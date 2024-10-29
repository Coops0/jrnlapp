import type { $Fetch } from 'nitropack';
import type { Profile } from '~/types/profile.type';

export class ProfileService {
    constructor(private readonly api: $Fetch) {
    };

    async putTimezone(tz: string): Promise<void> {
        return this.api('/users/timezone', { method: 'PUT', body: { timezone: tz } });
    }

    async getMe(): Promise<Profile> {
        return this.api('/users/me');
    }
}