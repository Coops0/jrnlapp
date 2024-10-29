import type { $Fetch } from 'nitropack';
import type { Entry } from '~/types/entry.type';

interface PaginatedResponse<T> {
    items: T[];
    next_cursor: string | null;
    has_more: boolean;
}

export class EntryService {
    constructor(private readonly api: $Fetch) {
    };

    async getEntriesPaginated(cursor?: string, limit?: number): Promise<PaginatedResponse<Entry>> {
        return this.api('/entries', { query: { cursor, limit } });
    }

    async getAverage(): Promise<number> {
        return this.api('/entries/average');
    }

    async getToday(): Promise<Entry> {
        return this.api('/entries/today');
    }

    async putToday(emotion_scale: number, text?: string): Promise<Entry> {
        return this.api('/entries/today', { method: 'PUT', body: { emotion_scale, text } });
    }
}