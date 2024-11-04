import type { $Fetch } from 'nitropack';
import type { User } from '~/types/user.type';

export class UserService {
    constructor(private readonly api: $Fetch) {
    };

    async updateMe(payload: { tz?: string; theme?: string; }): Promise<User> {
        return this.api('/user/me', { method: 'PATCH', body: payload });
    }

    async getMe(ignoreError?: boolean): Promise<User> {
        return this.api('/user/me', { ignoreError } as any);
    }
}