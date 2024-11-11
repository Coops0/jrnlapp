import type { $Fetch } from 'nitro/types';

export class AuthService {
    constructor(private readonly api: $Fetch) {
    }

    async logout(): Promise<void> {
        return this.api('/auth/logout');
    }

    async getSessionDetails(): Promise<{ csrf_token: string, nonce: string }> {
        return this.api('/auth/session');
    }
}