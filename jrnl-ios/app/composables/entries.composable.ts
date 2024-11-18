import type { EntryService, StrippedEntry } from '~/services/entry.service';
import { isSameDay, parseServerDate } from '~/util/index.util';
import type { LocalBackendService } from '~/services/local-backend.service';

type StrippedEntryWithDate = Omit<StrippedEntry, 'date'> & { date: Date };

export const useEntries = (localBackendService: LocalBackendService, entryService: EntryService | null) => {
    const nextCursor = ref<string | null>(null);
    const cursor = ref<string | null>(null);
    const hasMore = ref(false);

    const entries = ref<StrippedEntryWithDate[]>([]);

    const loadLocalEntries = async () => {
        const localEntries = await localBackendService.getEntries();
        entries.value = localEntries.map(e => ({ ...e, date: parseServerDate(e.date) }));
    };

    watch(cursor, async c => {
        if (!entryService) {
            return;
        }

        const paginator = await entryService.getEntriesPaginated(c || undefined, 50);

        const newEntries: StrippedEntryWithDate[] = [...(entries.value ?? [])];

        for (const entry of (paginator?.items ?? [])) {
            if (!newEntries.find((e: StrippedEntryWithDate) => e.id === entry.id)) {
                newEntries.push({
                    ...entry,
                    date: parseServerDate(entry.date)
                } as StrippedEntryWithDate);
            }
        }

        entries.value = newEntries.filter(e => !isSameDay(e.date));
        nextCursor.value = paginator?.next_cursor ?? null;
        hasMore.value = !!paginator?.has_more;
    }, { immediate: true });

    const loadMore = () => {
        if (hasMore.value && nextCursor.value) {
            cursor.value = nextCursor.value;
        }
    };

    return {
        loadMore,
        entries,
        loadLocalEntries,
        hasMore
    };
};