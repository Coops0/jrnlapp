import type { $Fetch } from 'nitro/types';

export class AuthService {
    constructor(private readonly api: $Fetch) {
    }

    async getSessionDetails(): Promise<{ csrf_token: string, nonce: string }> {
        return this.api('/auth/session');
    }

    async takeSession(nonce: string): Promise<string> {
        return this.api('/auth/take-session', {
            method: 'PATCH',
            nonce
        });
    }
}