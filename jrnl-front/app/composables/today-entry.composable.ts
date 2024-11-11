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

    const lastSaved = ref(new Date(1900, 1, 1));
    const tomorrow = ref(getTomorrow());

    const saveConflict = ref<[Entry, Entry] | null>(null);

    const updateTomorrowIntervalId = ref<NodeJS.Timeout | null>(null);
    const saveEntryTimeoutId = ref<NodeJS.Timeout | null>(null);

    const lastSavedEntry = ref<Entry | null>(null);
    const cancelledSaves = ref(0);

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

        if (lastSavedEntry.value && JSON.stringify(lastSavedEntry.value) === JSON.stringify(entry.value)) {
            console.log('cancelling save, no changes');
            cancelledSaves.value++;
            return;
        }

        if (saveEntryTimeoutId.value) {
            clearTimeout(saveEntryTimeoutId.value);
        }

        saveEntryTimeoutId.value = setTimeout(async () => {
            cancelledSaves.value = 0;
            try {
                await saveNow();
            } finally {
                saveEntryTimeoutId.value = null;
            }
        }, 300);
    }

    watch(entry, save, { deep: true });

    async function saveNow() {
        if (!entry.value || saveConflict.value) {
            return;
        }

        let entryResponse: Entry;
        try {
            entryResponse = await entryService.putToday(entry.value.emotion_scale, entry.value.text);
        } catch (e) {
            throw createError({
                message: 'failed to save entry',
                data: entry.value,
                cause: e,
            });
        }

        lastSavedEntry.value = { ...entry.value };
        console.debug('saved entry');

        storage.value = entryResponse;
        lastSaved.value = new Date();
    }

    const {
        status,
        execute: fetchToday
    } = useLazyAsyncData('today-entry-fetch', () => entryService.getToday(), {
        immediate: false,
        async transform(today) {
            lastSavedEntry.value = today && { ...today };

            if (today === null) {
                console.debug('fetch entry returned null, defaulting to blank');
                today = BLANK_ENTRY();
            }

            if (
                (JSON.stringify(today) !== JSON.stringify(entry.value)) &&
                (entry.value.text?.length || entry.value.emotion_scale !== 5)
            ) {
                onConflict(today);
                return today;
            }

            console.debug('setting entry to fetched state');

            entry.value = today;
            storage.value = today;

            return today;
        }
    });

    function onConflict(today: Entry) {
        console.warn('conflict detected between saved storage state && fetched state');
        console.debug(today, entry.value);

        saveConflict.value = [today, entry.value];
    }

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

        if (saveEntryTimeoutId.value) {
            clearTimeout(saveEntryTimeoutId.value);
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
        handleSaveConflict
    };
};