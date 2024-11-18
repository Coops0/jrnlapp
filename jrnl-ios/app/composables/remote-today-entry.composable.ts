import type { Entry } from '~/types/entry.type';
import type { EntryService } from '~/services/entry.service';
import { getTomorrow, isSameDay, parseServerDate } from '~/util/index.util';
import { BLANK_ENTRY } from '~/composables/local-today-entry.composable';

export const useRemoteTodayEntry = (entryService: EntryService, entry: Ref<Entry>) => {
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
        if (!entryService) {
            await saveNow();
            return;
        }

        if (cancelledSaves.value >= 20) {
            console.debug('too many cancelled saves, saving forcefully');

            cancelledSaves.value = 0;
            await saveNow();
            return;
        }

        if (lastSavedEntry.value && JSON.stringify(lastSavedEntry.value) === JSON.stringify(entry.value)) {
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
        // todo save api to backend

        if (entryService) {
            entry.value = await entryService.putToday(entry.value.emotion_scale, entry.value.text);
        }

        lastSavedEntry.value = { ...entry.value };
        lastSaved.value = new Date();
    }

    const status = ref<string>('pending');

    async function fetchToday() {
        let today: Entry | null = null;
        try {
            today = await entryService.getToday();
        } catch (e) {
            status.value = 'error';
            console.error('error fetching today', e);
            throw e;
        }

        status.value = 'success';
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

        return today;
    }

    const mounted = async () => {
        // if day changes as we are writing, then reset too
        updateTomorrowIntervalId.value = setInterval(async () => {
            if (!isSameDay(tomorrow.value)) {
                // tomorrow day is still different, we are good
                return;
            }

            console.debug('tripped daily reset');
            tomorrow.value = getTomorrow();

            // todo save to local entries

            entry.value = BLANK_ENTRY();
        }, 1000);

        if (status.value === 'success') {
            console.debug('already fetched, skipping initial cookie load');
            return;
        }

        if (!isSameDay(parseServerDate(entry.value.date))) {
            entry.value = BLANK_ENTRY();
            return;
        }
    };


    const unMounted = () => {
        if (updateTomorrowIntervalId.value) {
            clearInterval(updateTomorrowIntervalId.value);
        }

        if (debouncedSaveTimeout.value) {
            clearTimeout(debouncedSaveTimeout.value);
        }
    };

    async function handleSaveConflict(server: boolean) {
        if (!saveConflict.value) {
            return;
        }

        const [today,] = saveConflict.value;
        saveConflict.value = null;

        if (server) {
            entry.value = today;
            // todo
        } else {
            await saveNow();
        }
    }

    return {
        fetchToday,
        tomorrow,

        lastSaved,
        lastSavedEntry,

        saveConflict,
        handleSaveConflict,

        forceSave: saveNow,
        unsavedChanges,

        mounted,
        unMounted
    };
};