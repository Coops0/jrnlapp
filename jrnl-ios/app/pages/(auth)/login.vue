<template>
  <div class="size-full flex-grow flex items-center justify-center place-self-center p-6">
    <div class="w-full max-w-md">
      <div class="bg-colors-primary-800/50 rounded-xl p-8 backdrop-blur-sm lg:scale-125">
        <div class="space-y-6">
          <div class="flex flex-col items-center gap-3">
            <LoginGoogleButton class="w-full h-[40px]" @click="startGoogleLogin"/>
            <LoginAppleButton class="w-full h-[40px]" @click="startAppleLogin"/>
          </div>

          <div
              v-if="error"
              class="bg-colors-primary-800/50 rounded-xl p-8 backdrop-blur-sm border border-colors-primary-700 text-center"
          >
            <div class="text-red-400">
              <span class="text-lg">login failed</span>
            </div>
            <p class="text-colors-text-300 mt-2">{{ error }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { AuthService, type ServerResponse } from '~/services/auth.service';
import { UserService } from '~/services/user.service';
import { get_apple_id_credential } from 'tauri-plugin-sign-in-with-apple-api';
import { open as tauriOpen } from '@tauri-apps/plugin-shell';
import { onOpenUrl } from '@tauri-apps/plugin-deep-link';

const { public: { apiBase, googleClientId } } = useRuntimeConfig();
const { $localApi } = useNuxtApp();

const authService = new AuthService($localApi);
const userService = new UserService($localApi);

const { jwt } = await useAuth();
const { user, hasRefreshedRemotely } = await useUser(userService);

const { data: sessionDetails } = await useAsyncData('session-details', () => authService.getSessionDetails());

const error = ref<string | null>(null);

async function startGoogleLogin() {
  await onOpenUrl(async ([url]) => {
    if (url?.startsWith(`${apiBase}/auth/apple/deeplink`) !== true) {
      return;
    }

    const uri = new URL(url);
    let decodedServerResponse: ServerResponse;
    try {
      decodedServerResponse = JSON.parse(atob(uri.searchParams.get('r')!));
    } catch (e) {
      console.error(e);
      error.value = 'failed to login to google';
      return;
    }

    await handleServerResponse(decodedServerResponse);
  });

  await tauriOpen(`https://accounts.google.com/o/oauth2/v2/auth?client_id=${googleClientId}&redirect_uri=${apiBase}/auth/google/redirect&response_type=code&scope=openid%20profile`);
}

async function startAppleLogin() {
  let response;
  try {
    response = await get_apple_id_credential({
      scope: ['fullName'],
      nonce: sessionDetails.value!.nonce,
      state: sessionDetails.value!.csrf_token
    });
  } catch (e) {
    console.error(e);
    error.value = 'failed to login with apple';
    return;
  }

  const correctlyFormattedResponse = {
    authorization: {
      id_token: response.identityToken,
      state: response.state
    },
    user: response.givenName && {
      name: {
        firstName: response.givenName
      }
    }
  };

  let serverResponse: ServerResponse;
  try {
    serverResponse = await authService.loginWithApple(correctlyFormattedResponse);
  } catch (e) {
    console.error(e);
    error.value = 'failed to login to jrnl with apple';
    return;
  }

  await handleServerResponse(serverResponse);
}

async function handleServerResponse(response: ServerResponse) {
  if ('error' in response) {
    console.error(response.error);
    error.value = response.error;
    return;
  }

  jwt.value = response.token;

  hasRefreshedRemotely.value = true;
  user.value = response.user;

  try {
    await userService.updateMe({ tz: Intl.DateTimeFormat().resolvedOptions().timeZone });
  } catch (e) {
    console.warn(e);
  }

  await navigateTo('/current');
}

</script>
