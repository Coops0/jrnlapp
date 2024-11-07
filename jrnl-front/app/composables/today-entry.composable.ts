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

    const save = useDebounceFn(
        async () => {
            console.debug('saved debounce fn called');
            if (!entry.value) return;

            let entryResponse: Entry;
            try {
                entryResponse = await entryService.putToday(entry.value.emotion_scale, entry.value.text);
            } catch (e) {
                alert('failed to save entry');
                console.error(e);
                return;
            }
            console.debug('saved entry');

            storage.value = entryResponse;
            lastSaved.value = new Date();
        },
        350,
        { maxWait: 2500 }
    );

    const { ignoreUpdates } = watchIgnorable(entry, async e => {
        if (e) {
            storage.value = e;
        }
        await save();
    }, { deep: true });


    const {
        status,
        execute: fetchToday
    } = useLazyAsyncData('today-entry-fetch', () => entryService.getToday(), {
        immediate: false,
        async transform(today) {
            if (today === null) {
                console.debug('fetch entry returned null, defaulting to blank');
                today = BLANK_ENTRY();
            }

            if (
                (JSON.stringify(today) !== JSON.stringify(entry.value)) &&
                (entry.value.text?.length || entry.value.emotion_scale !== 5)
            ) {
                console.warn('conflict detected between saved storage state && fetched state... defaulting to local storage');
                console.debug(today, entry.value);
                await save();
                return today;
            }

            console.debug('setting entry to fetched state');
            ignoreUpdates(() => {
                entry.value = today;
            });

            storage.value = today;
            return today;
        }
    });

    onMounted(() => {
        if (status.value === 'success') {
            console.debug('already fetched, skipping initial local storage check');
            return;
        }

        if (!storage.value) {
            console.debug('no local storage');
            return;
        }

        if (!isSameDay(parseServerDate(storage.value.date))) {
            console.debug('resetting local storage', storage.value, parseServerDate(storage.value.date));
            storage.value = BLANK_ENTRY();
            return;
        }

        console.debug('loading from local storage');

        ignoreUpdates(() => {
            entry.value = storage.value;
        });
    });

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