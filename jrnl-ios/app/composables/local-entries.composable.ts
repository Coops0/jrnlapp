import { load } from '@tauri-apps/plugin-store';
import type { Entry } from '~/types/entry.type';
import { isSameDay } from '~/util/index.util';

export const useLocalEntries = async () => {
    const storage = await load('local-entries');
    const initialCachedEntries = await storage.get<Entry[]>('entries');

    const entries = useState<Entry[]>('entries', () => initialCachedEntries ?? []);

    async function saveEntry(entry: Entry) {
        const index = entries.value.findIndex(e => e.id === entry.id);
        if (index === -1) {
            entries.value.push(entry);
        } else {
            entries.value[index] = entry;
        }

        sort();
        await storage.set('entries', entries.value);
    }

    function sort() {
        entries.value
            .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    }

    return { entries, saveEntry };
};