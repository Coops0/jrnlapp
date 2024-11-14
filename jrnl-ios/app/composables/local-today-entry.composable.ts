import type { Entry } from '~/types/entry.type';
import { getTomorrow, isSameDay, parseServerDate } from '~/util/index.util';
import type { Store } from '@tauri-apps/plugin-store';

const BLANK_ENTRY = (): Entry => ({
    text: '',
    emotion_scale: 5,
    date: new Date().toString(),
    id: crypto.randomUUID(),
    author: '',
});

export const useLocalTodayEntry = async (storage: Store) => {
    const { saveEntry } = await useLocalEntries();
    const entry = ref<Entry>(await storage.get('entry') ?? BLANK_ENTRY());

    const tomorrow = ref(getTomorrow());
    const updateTomorrowIntervalId = ref<NodeJS.Timeout | null>(null);

    async function save() {
        await Promise.all([
            storage.set('entry', entry.value),
            saveEntry(entry.value)
        ]);
    }

    watch(entry, save, { deep: true });

    onMounted(async () => {
        const s: Entry | undefined = await storage.get('entry');

        if (!s) {
            return;
        }

        if (!isSameDay(parseServerDate(s.date))) {
            await storage.set('entry', BLANK_ENTRY());
            return;
        }

        entry.value = s;
    });

    onMounted(() => {
        updateTomorrowIntervalId.value = setInterval(async () => {
            if (!isSameDay(tomorrow.value)) {
                // tomorrow day is still different, we are good
                return;
            }

            console.debug('tripped daily reset');
            tomorrow.value = getTomorrow();

            await saveEntry(entry.value);

            entry.value = BLANK_ENTRY();
            await storage.set('entry', BLANK_ENTRY());
        }, 1000);
    });

    onUnmounted(() => {
        if (updateTomorrowIntervalId.value) {
            clearInterval(updateTomorrowIntervalId.value);
        }
    });

    return { entry, tomorrow };
};