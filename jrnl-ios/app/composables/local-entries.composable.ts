import type { Entry } from '~/types/entry.type';

export const useLocalEntries = () => {
    // const storage = await load('local-entries');
    // const initialCachedEntries = await storage.get<Entry[]>('entries');
// todo
    const entries = useState<Entry[]>('entries', () => []);

    async function saveEntry(entry: Entry) {
        const index = entries.value.findIndex(e => e.id === entry.id);
        if (index === -1) {
            entries.value.push(entry);
        } else {
            entries.value[index] = entry;
        }

        sort();
    }

    function sort() {
        entries.value
            .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
    }

    return { entries, saveEntry };
};