import type { $Fetch } from 'nitropack';
import type { Profile } from '~/types/profile.type';

export class ProfileService {
    constructor(private readonly api: $Fetch) {
    };

    async updateMe(payload: { tz?: string; favorite_color?: string; }): Promise<Profile> {
        return this.api('/user/me', { method: 'PATCH', body: payload });
    }

    async getMe(): Promise<Profile> {
        return this.api('/user/me');
    }
}