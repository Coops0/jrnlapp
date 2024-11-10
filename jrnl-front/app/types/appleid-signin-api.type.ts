// https://github.com/DefinitelyTyped/DefinitelyTyped/blob/master/types/apple-signin-api/index.d.ts

// https://developer.apple.com/documentation/signinwithapplejs/authorizationi
export interface AuthorizationI {
    code: string;
    id_token: string;
    state: string;
    nonce?: string | undefined;
}

// https://developer.apple.com/documentation/signinwithapplejs/namei
export interface NameI {
    firstName: string;
    lastName: string;
}

// https://developer.apple.com/documentation/signinwithapplejs/signinerrori
export interface SignInErrorI {
    error: string;
}

// https://developer.apple.com/documentation/signinwithapplejs/signinresponsei
export interface SignInResponseI {
    authorization: AuthorizationI;
    user?: UserI | undefined;
}

// https://developer.apple.com/documentation/signinwithapplejs/useri
export interface UserI {
    email: string;
    name: NameI;
}

// https://developer.apple.com/documentation/signinwithapplejs/authi
export interface AuthI {
    init: (config: ClientConfigI) => void;
    signIn: (signInConfig?: ClientConfigI) => Promise<SignInResponseI>;
    renderButton: () => void;
}

// https://developer.apple.com/documentation/signinwithapplejs/clientconfigi
export interface ClientConfigI {
    clientId?: string | undefined;
    redirectURI?: string | undefined;
    scope?: string | undefined;
    state?: string | undefined;
    nonce?: string | undefined;
    usePopup?: boolean | undefined;
}

export interface AppleID {
    auth: AuthI;
}