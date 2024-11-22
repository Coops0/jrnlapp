import type { LocalBackendService } from '~/services/local-backend.service';
import { useOnline } from '~/composables/util/online.util.composable';

type EntryWithDate = Omit<Entry, 'date'> & { date: Date };

export const useEntries = (localBackendService: LocalBackendService, entryService: EntryService) => {
    const { isConnected } = useOnline();

    const nextCursor = ref<string | null>(null);
    const cursor = ref<string | null>(null);
    const hasMore = ref(false);

    const entries = ref<EntryWithDate[]>([]);

    const loadLocalEntries = async () => {
        const localEntries = await localBackendService.getEntries();
        entries.value = localEntries.map(e => ({ ...e, date: parseServerDate(e.date) }));
    };

    async function loadRemoteEntries() {
        const paginator = await entryService.getEntriesPaginated(cursor.value || undefined, 50);
        const newEntries: EntryWithDate[] = [...(entries.value ?? [])];

        for (const entry of (paginator?.items ?? [])) {
            if (!newEntries.find((e: EntryWithDate) => e.id === entry.id)) {
                newEntries.push({
                    ...entry,
                    date: parseServerDate(entry.date),
                    saved: true,
                } as EntryWithDate);
            }
        }

        entries.value = newEntries.filter(e => !isSameDay(e.date));
        nextCursor.value = paginator?.next_cursor ?? null;
        hasMore.value = !!paginator?.has_more;
    }

    watch(isConnected, () => {
        if (isConnected.value) {
            loadRemoteEntries()
                .catch(() => loadLocalEntries());
        } else {
            void loadLocalEntries();
        }
    }, { immediate: true });

    const loadMore = async () => {
        if (hasMore.value && nextCursor.value) {
            cursor.value = nextCursor.value;
            await loadRemoteEntries();
        }
    };

    return {
        loadMore,
        entries,
        hasMore
    };
};