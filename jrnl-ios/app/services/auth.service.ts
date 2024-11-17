import type { $Fetch } from 'nitro/types';
import type { User } from '~/types/user.type';

export type ServerResponse = { error: string } | { user: User, token: string };

export class AuthService {
    constructor(private readonly api: $Fetch) {
    }

    async getSessionDetails(): Promise<{ csrf_token: string, nonce: string }> {
        return this.api('/auth/session');
    }

    async loginWithApple(payload: unknown): Promise<ServerResponse>  {
        return this.api('/auth/apple', {
            method: 'POST',
            body: payload as object
        });
    }
}