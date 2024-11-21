import type { Entry } from '~/types/entry.type';
import type { EntryService } from '~/services/entry.service';
import { getTomorrow, isSameDay, parseServerDate } from '~/util/index.util';
import type { CookieRef } from '#app';

const BLANK_ENTRY = (): Entry => ({
    text: '',
    emotion_scale: 5,
    date: new Date().toString(),
    author: '',
    id: ''
});

export const useTodayEntry = (entryService: EntryService, storage: CookieRef<Entry>) => {
    const entry = ref<Entry>(storage.value ?? BLANK_ENTRY());

    const lastSavedEntry = ref<Entry | null>(null);
    const lastSaved = ref(new Date(1900, 1, 1));

    const tomorrow = ref(getTomorrow());

    const saveConflict = ref<[Entry, Entry] | null>(null);

    const debouncedSaveTimeout = ref<NodeJS.Timeout | null>(null);
    const cancelledSaves = ref(0);

    const updateTomorrowIntervalId = ref<NodeJS.Timeout | null>(null);

    const unsavedChanges = computed(() =>
        !!((!lastSavedEntry.value || JSON.stringify(lastSavedEntry.value) !== JSON.stringify(entry.value)) &&
            (entry.value.text?.length || entry.value.emotion_scale !== 5))
    );

    async function save() {
        storage.value = {
            ...storage.value,
            text: entry.value.text,
            emotion_scale: entry.value.emotion_scale
        };

        if (cancelledSaves.value >= 20) {
            console.debug('too many cancelled saves, saving forcefully');

            cancelledSaves.value = 0;
            await saveNow();
            return;
        }

        if (!unsavedChanges.value) {
            console.log('cancelling save, no changes');
            cancelledSaves.value++;
            return;
        }

        if (debouncedSaveTimeout.value) {
            clearTimeout(debouncedSaveTimeout.value);
        }

        debouncedSaveTimeout.value = setTimeout(async () => {
            cancelledSaves.value = 0;
            try {
                await saveNow();
            } finally {
                debouncedSaveTimeout.value = null;
            }
        }, 300);
    }

    watch(entry, save, { deep: true });

    async function saveNow() {
        if (!entry.value || saveConflict.value) {
            return;
        }

        const entryResponse = await entryService.putToday(entry.value.emotion_scale, entry.value.text);

        lastSavedEntry.value = { ...entry.value };

        storage.value = entryResponse;
        lastSaved.value = new Date();
    }

    const {
        status,
        execute: fetchToday
    } = useLazyAsyncData('today-entry-fetch', () => entryService.getToday(), {
        immediate: false,
        transform(today) {
            lastSavedEntry.value = today && { ...today };

            if (today === null) {
                console.debug('fetch entry returned null, defaulting to blank');
                today = BLANK_ENTRY();
            }

            if (
                (JSON.stringify(today) !== JSON.stringify(entry.value)) &&
                (entry.value.text?.length || entry.value.emotion_scale !== 5)
            ) {
                console.warn('conflict detected between saved storage state && fetched state');
                console.debug(today, entry.value);

                saveConflict.value = [today, entry.value];
                return today;
            }

            console.debug('setting entry to fetched state');

            entry.value = today;
            storage.value = today;

            return today;
        }
    });

    onMounted(() => {
        if (status.value === 'success') {
            console.debug('already fetched, skipping initial cookie load');
            return;
        }

        if (!storage.value) {
            console.debug('no cached entry');
            return;
        }

        if (!isSameDay(parseServerDate(storage.value.date))) {
            console.debug('resetting local entry, different day', storage.value, parseServerDate(storage.value.date));
            storage.value = BLANK_ENTRY();
            return;
        }

        console.debug('loading cached entry');
        entry.value = storage.value;
    });

    onMounted(() => {
        // if day changes as we are writing, then reset too
        updateTomorrowIntervalId.value = setInterval(() => {
            if (!isSameDay(tomorrow.value)) {
                // tomorrow day is still different, we are good
                return;
            }

            console.debug('tripped daily reset');
            tomorrow.value = getTomorrow();

            entry.value = BLANK_ENTRY();
            storage.value = BLANK_ENTRY();
        }, 1000);
    });

    onUnmounted(() => {
        if (updateTomorrowIntervalId.value) {
            clearInterval(updateTomorrowIntervalId.value);
        }

        if (debouncedSaveTimeout.value) {
            clearTimeout(debouncedSaveTimeout.value);
        }
    });

    async function handleSaveConflict(server: boolean) {
        if (!saveConflict.value) {
            return;
        }

        const [today,] = saveConflict.value;
        saveConflict.value = null;

        if (server) {
            entry.value = today;
            storage.value = today;
        } else {
            await saveNow();
        }
    }

    return {
        entry,
        beginFetch: fetchToday,

        tomorrow,

        lastSaved,
        lastSavedEntry,

        saveConflict,
        handleSaveConflict,

        forceSave: saveNow,
        unsavedChanges
    };
};