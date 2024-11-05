import type { $Fetch } from 'nitropack';
import type { User } from '~/types/user.type';

export class AuthService {
    constructor(private readonly api: $Fetch) {
    }

    async logout(): Promise<void> {
        return this.api('/auth/logout');
    }

    async getGoogleRedirect(): Promise<ReturnType<$Fetch>> {
        return this.api('/auth/google');
    }

    async loginWithGoogle(state: string, code: string): Promise<{ user: User, token: string }> {
        return this.api('/auth/google/callback', {
            ignoreResponseError: true,
            method: 'POST',
            body: { state, code }
        });
    }
}