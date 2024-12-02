import type { $Fetch } from 'nitro/types';
import type { User } from '~/types/user.type';

export class UserService {
    constructor(private readonly api: $Fetch) {
    };

    async updateMe(payload: { tz?: string; theme?: string; has_had_tour?: boolean; has_seen_app_push?: boolean; }): Promise<User> {
        return this.api('/user/me', { method: 'PATCH', body: payload });
    }

    async getMe(): Promise<User> {
        return this.api('/user/me');
    }
}