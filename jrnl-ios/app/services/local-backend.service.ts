import type { Entry } from '~/types/entry.type';
import { invoke } from '@tauri-apps/api/core';

export class LocalBackendService {
    async saveEntry(entry: Entry): Promise<void> {
        await invoke('save_entry', { entry });
    }

    async getEntry(id: string): Promise<Entry | null> {
        return await invoke('get_entry', { id });
    }

    async getEntries(): Promise<Entry[]> {
        return await invoke('get_entries');
    }
}