import type { StrippedEntry, EntryService } from '~/services/entry.service';
import { isSameDay, parseServerDate } from '~/util/index.util';

type StrippedEntryWithDate = Omit<StrippedEntry, 'date'> & { date: Date };

export const useRemoteEntries = (entryService: EntryService) => {
    const nextCursor = ref<string | null>(null);
    const limit = ref(50);

    const additionalLoads = ref(0);

    const { data: paginator, status, error, refresh } = useLazyAsyncData(
        'entries',
        () => entryService.getEntriesPaginated(nextCursor.value || undefined, limit.value),
        {
            watch: [nextCursor],
            transform(p) {
                const entries: StrippedEntryWithDate[] = [...(paginator.value?.items ?? [])];

                for (const entry of (p?.items ?? [])) {
                    if (!entries.find((e: StrippedEntryWithDate) => e.id === entry.id)) {
                        entries.push({
                            ...entry,
                            date: parseServerDate(entry.date)
                        } as StrippedEntryWithDate);
                    }
                }

                return {
                    ...p,
                    // don't show today entry
                    items: entries.filter(e => !isSameDay(e.date))
                };
            }
        }
    );

    function loadMore() {
        if (paginator.value?.has_more && paginator.value?.next_cursor) {
            if (++additionalLoads.value > 3) {
                limit.value = 100;
            }

            nextCursor.value = paginator.value.next_cursor;
        }
    }

    return {
        paginator,
        status,
        error,
        refresh,
        loadMore
    };
};