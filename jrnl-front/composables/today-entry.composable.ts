import type { Entry } from '~/types/entry.type';
import type { EntryService } from '~/services/entry.service';
import { getTomorrow, isSameDay, parseServerDate } from '~/util/index.util';

const BLANK_ENTRY = (): Entry => ({
    text: '',
    emotion_scale: 5,
    date: new Date().toString(),
    author: '',
    id: ''
});

export const useTodayEntry = (entryService: EntryService) => {
    const entry = ref<Entry>(BLANK_ENTRY());

    const lastSaved = ref(new Date(1900, 1, 1));
    const tomorrow = ref(getTomorrow());

    const storage = useCookie('entry-today', {
        default() {
            return BLANK_ENTRY();
        }
    });

    const save = useDebounceFn(async () => {
        console.debug('saved debounce fn called');
        if (!entry.value) return;

        storage.value = entry.value;
        await entryService.putToday(entry.value.emotion_scale, entry.value.text);
        console.debug('saved entry');

        lastSaved.value = new Date();
    }, 600);

    const { ignoreUpdates } = watchIgnorable(entry, save, { deep: true });

    const {
        data: todayFetchLazy,
        execute: fetchToday
    } = useLazyAsyncData('today-entry-fetch', entryService.getToday, { immediate: false });

    const wasCachedEntryValid = ref<boolean>(false);

    onMounted(() => {
        if (todayFetchLazy.value?.id) {
            console.debug('already fetched, skipping initial local storage check');
            return;
        }

        if (isSameDay(parseServerDate(storage.value.date))) {
            console.debug('loading from local storage');

            wasCachedEntryValid.value = true;
            ignoreUpdates(() => {
                entry.value = storage.value;
            });
        } else {
            console.debug('resetting local storage', storage.value, parseServerDate(storage.value.date));
            storage.value = BLANK_ENTRY();
        }
    });

    watchOnce(todayFetchLazy, async today => {
        if (today === null) {
            console.debug('fetch entry returned null, defaulting to blank');
            today = BLANK_ENTRY();
        }

        try {
            if (wasCachedEntryValid.value && JSON.stringify(today) !== JSON.stringify(entry.value)) {
                console.warn('conflict detected between saved storage state && fetched state... defaulting to local storage');
                await save();
                return;
            }
        } catch (e) {
            console.warn('error comparing storage and fetched state', e);
        }

        console.debug('setting entry to fetched state');
        ignoreUpdates(() => {
            entry.value = today;
        });

        storage.value = today;
    }, { deep: true });

    // if day changes as we are writing, then reset too
    useIntervalFn(() => {
        if (!isSameDay(tomorrow.value)) {
            // tomorrow day is still different, we are good
            return;
        }

        console.debug('tripped daily reset');
        tomorrow.value = getTomorrow();

        ignoreUpdates(() => {
            entry.value = BLANK_ENTRY();
        });

        storage.value = BLANK_ENTRY();
    }, 1000);


    return {
        entry,
        lastSaved,
        beginFetch: fetchToday,
        tomorrow
    };
};