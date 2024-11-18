import type { Entry } from '~/types/entry.type';
import { getTomorrow, isSameDay, parseServerDate } from '~/util/index.util';

export const BLANK_ENTRY = (): Entry => ({
    text: '',
    emotion_scale: 5,
    date: new Date().toString(),
    id: crypto.randomUUID(),
    author: '',
});

export const useLocalTodayEntry = (entry: Ref<Entry>) => {
    const { saveEntry } = useLocalEntries(); // todo

    const tomorrow = ref(getTomorrow());
    const updateTomorrowIntervalId = ref<NodeJS.Timeout | null>(null);

    watch(entry, async (e) => saveEntry(e), { deep: true });

    const mounted = async () => {
        updateTomorrowIntervalId.value = setInterval(async () => {
            if (!isSameDay(tomorrow.value)) {
                // tomorrow day is still different, we are good
                return;
            }

            console.debug('tripped daily reset');
            tomorrow.value = getTomorrow();

            await saveEntry(entry.value);

            entry.value = BLANK_ENTRY();
        }, 1000);

        if (!isSameDay(parseServerDate(entry.value.date))) {
            entry.value = BLANK_ENTRY();
        }
    };

    const unMounted = () => {
        if (updateTomorrowIntervalId.value) {
            clearInterval(updateTomorrowIntervalId.value);
        }
    };

    return { entry, tomorrow, mounted, unMounted };
};