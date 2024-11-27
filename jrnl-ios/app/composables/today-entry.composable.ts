import type { Entry } from '~/types/entry.type';
import type { EntryService } from '~/services/entry.service';
import { getTomorrow, isSameDay, parseServerDate } from '~/util/index.util';
import { useDebouncedFn } from '~/composables/util/debounced-fn.util.composable';
import type { LocalBackendService } from '~/services/local-backend.service';
import { useOnline } from '~/composables/util/online.util.composable';

export const BLANK_ENTRY = (): Entry => ({
    text: '',
    emotion_scale: 5,
    date: new Date().toLocaleDateString(),
    id: crypto.randomUUID(),
    author: '',
    saved: false,
    ephemeral: false
});

export const useTodayEntry = (
    entry: Ref<Entry>,
    localBackendService: LocalBackendService,
    entryService: EntryService,
) => {
    const { isConnected } = useOnline();
    const lastSavedEntry = ref<Entry | null>(null);
    const lastSaved = ref(new Date(1900, 1, 1));

    const tomorrow = ref(getTomorrow());
    const showDayTransition = ref(false);

    const saveConflict = ref<[Entry, Entry] | null>(null);

    const updateTomorrowIntervalId = ref<NodeJS.Timeout | null>(null);

    const unsavedChanges = computed<boolean>(() => {
        if (!lastSavedEntry.value || !isConnected.value) {
            return false;
        }

        if (JSON.stringify(lastSavedEntry.value) !== JSON.stringify(entry.value)) {
            return true;
        }

        return !!(entry.value.text?.length || entry.value.emotion_scale !== 5);
    });

    async function saveNow() {
        if (!entry.value || saveConflict.value) {
            return;
        }

        try {
            if (isConnected.value) {
                entry.value = await entryService.putToday(entry.value.emotion_scale, entry.value.text!, entry.value.ephemeral);
                entry.value.saved = true;
            }
        } catch (e) {
            console.error('error saving entry', e);
        } finally {
            await localBackendService.saveEntry(entry.value);
        }

        lastSavedEntry.value = { ...entry.value };
        lastSaved.value = new Date();
    }

    const { debounced: saveDebounced } = useDebouncedFn(() => saveNow());

    // with debounced
    async function save() {
        if (!isConnected.value) {
            await saveNow();
        } else if (!lastSavedEntry.value || unsavedChanges.value) {
            saveDebounced();
        }
    }

    watch(entry, () => save(), { deep: true });

    const status = ref<string>('pending');

    async function fetchToday() {
        if (!isConnected.value) {
            return;
        }

        let today: Entry | null = null;
        try {
            today = await entryService.getToday();
        } catch (e) {
            status.value = 'error';
            console.error('error fetching today', e);
            throw e;
        }

        if (today) {
            today.saved = true;
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
            return;
        }

        entry.value = today;
    }

    onMounted(() => {
        // if day changes as we are writing, then reset too
        updateTomorrowIntervalId.value = setInterval(async () => {
            if (!isSameDay(tomorrow.value)) {
                return;
            }

            if (unsavedChanges.value) {
                await saveNow();
            }

            await localBackendService.saveEntry(entry.value);

            tomorrow.value = getTomorrow();

            showDayTransition.value = true;

            setTimeout(() => {
                entry.value = BLANK_ENTRY();
                lastSavedEntry.value = null;
            }, 3000);
        }, 1000);

        if (status.value === 'success') {
            console.debug('already fetched, skipping initial cookie load');
        } else if (!isSameDay(parseServerDate(entry.value.date))) {
            entry.value = BLANK_ENTRY();
        }
    });

    onUnmounted(() => {
        if (updateTomorrowIntervalId.value) {
            clearInterval(updateTomorrowIntervalId.value);
        }
    });

    onBeforeUnmount(async () => {
        if (unsavedChanges.value) {
            await saveNow();
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
            await localBackendService.saveEntry(today);
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

        saveNow,
        unsavedChanges,
        showDayTransition
    };
};