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
    const tomorrow = ref(new Date());

    const save = useDebounceFn(async () => {
        if (!entry.value) {
            return;
        }

        storage.value = entry.value;
        await entryService.putToday(entry.value.emotion_scale, entry.value.text);

        lastSaved.value = new Date();
    }, 600);


    const { pause, resume } = pausableWatch(entry, save);
    // immediately pause as to not save on initial load
    pause();


    async function beginFetch() {
        try {
            const today = await entryService.getToday();

            if (entry.value.text === storage.value.text && entry.value.emotion_scale === storage.value.emotion_scale) {
                // no modifications have been made, we can overwrite
                entry.value = today;
                storage.value = today;
            }
        } finally {
            resume();
        }
    }

    // if day changes as we are writing, then reset too
    useIntervalFn(() => {
        if (!isSameDay(tomorrow.value)) {
            // tomorrow day is still different, we are good
            return;
        }

        tomorrow.value = getTomorrow();
        pause();
        entry.value = BLANK_ENTRY();
        resume();

        storage.value = BLANK_ENTRY();
    }, 1000);


    return {
        entry,
        lastSaved,
        beginFetch,
        tomorrow
    };
};