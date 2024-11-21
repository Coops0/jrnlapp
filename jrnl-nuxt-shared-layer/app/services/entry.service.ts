import type { $Fetch } from 'nitro/types';
import type { Entry } from '~/types/entry.type';

export interface PaginatedResponse<T> {
    items: T[];
    next_cursor: string | null;
    has_more: boolean;
}

export type StrippedEntry = Omit<Entry, 'text' | 'author'>;

export class EntryService {
    constructor(private readonly api: $Fetch) {
    };

    async getEntriesPaginated(cursor?: string, limit?: number): Promise<PaginatedResponse<StrippedEntry>> {
        return this.api('/entries', { query: { cursor, limit } });
    }

    async getEntry(id: string): Promise<Entry | null> {
        return this.api(`/entries/${id}`);
    }

    async getAverage(): Promise<number> {
        return this.api('/entries/average');
    }

    async getToday(): Promise<Entry | null> {
        return this.api('/entries/today');
    }

    async putToday(emotion_scale: number, text: string, ephemeral: boolean): Promise<Entry> {
        return this.api('/entries/today', { method: 'PUT', body: { emotion_scale, text, ephemeral } });
    }

    async putPastEntries(entries: Entry[]): Promise<void> {
        return this.api('/entries', { method: 'PUT', body: entries });
    }
}