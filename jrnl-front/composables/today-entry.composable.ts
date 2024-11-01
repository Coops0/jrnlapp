import type { Entry } from '~/types/entry.type';
import type { EntryService } from '~/services/entry.service';
import { getTomorrow, isSameDay } from '~/util/index.util';

const BLANK_ENTRY = (): Entry => ({
    text: '',
    emotion_scale: 5,
    date: new Date().toString(),
    author: '',
    id: ''
});

export const useTodayEntry = (entryService: EntryService) => {
    const storage = useLocalStorage('entry-today', BLANK_ENTRY());
    const entry = ref<Entry>(storage.value);

    const lastSaved = ref(new Date(1900, 1, 1));
    const tomorrow = ref(getTomorrow());

    const save = useDebounceFn(async () => {
        console.debug('saved debounce fn called');
        if (!entry.value) {
            return;
        }

        storage.value = entry.value;
        await entryService.putToday(entry.value.emotion_scale, entry.value.text);
        console.debug('saved entry');

        lastSaved.value = new Date();
    }, 600);


    const { ignoreUpdates } = watchIgnorable(entry, () => save(), { deep: true });

    async function beginFetch() {
        try {
            const today = await entryService.getToday();
            today.text = today.text || '';

            if (entry.value.text === storage.value.text && entry.value.emotion_scale === storage.value.emotion_scale) {
                // no modifications have been made, we can overwrite
                ignoreUpdates(() => {
                    entry.value = today;
                });

                storage.value = today;
            }
        } finally {
            console.debug('finished initial entry fetch');
        }
    }

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
        beginFetch,
        tomorrow
    };
};