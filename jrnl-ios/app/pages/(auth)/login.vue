<template>
  <div class="size-full flex-grow flex items-center justify-center place-self-center p-6">
    <div class="w-full max-w-md">
      <div class="bg-colors-primary-800/50 rounded-xl p-8 backdrop-blur-sm lg:scale-125">
        <div class="space-y-6">
         <ErrorDisplay v-if="sessionError" :error="sessionError" :clear-error="refresh"/>
          <div v-else>
            <div class="flex flex-col items-center gap-3">
              <LoginGoogleButton class="w-full h-[40px]" @click="startGoogleLogin"/>
              <LoginAppleButton class="w-full h-[40px]" @click="startAppleLogin"/>
            </div>

            <ErrorDisplay v-if="error" :error="error"/>
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
import { requestSignin } from 'tauri-plugin-google-signin-api';
import { LocalBackendService } from '~/services/local-backend.service';
import { EntryService } from '~/services/entry.service';

const { $localApi } = useNuxtApp();

const authService = new AuthService($localApi);
const userService = new UserService($localApi);

const { jwt } = useAuth();
const { user, hasRefreshedRemotely } = useUser(userService);

const {
  data: sessionDetails,
  error: sessionError,
  refresh
} = await useAsyncData('session-details', () => authService.getSessionDetails());

const error = ref<string | null>(null);

async function startGoogleLogin() {
  let response;
  try {
    response = await requestSignin(sessionDetails.value!.nonce);
  } catch (e: unknown) {
    console.error(e);
    error.value = e as string ?? 'failed to login with google';
    return;
  }

  let serverResponse: ServerResponse;
  try {
    serverResponse = await authService.loginWithGoogle({
      credential: response.idToken,
      state: sessionDetails.value!.csrf_token
    });
  } catch (e) {
    console.error(e);
    error.value = 'failed to login to jrnl with google';
    return;
  }

  await handleServerResponse(serverResponse);
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

  const entryService = new EntryService($localApi);

  try {
    const { entries } = useEntries(new LocalBackendService(), entryService);
    const unsaved = entries.value
        .filter(entry => !entry.saved)
        .map(entry => ({ ...entry, saved: true, date: entry.date.toLocaleDateString() }));

    if (unsaved.length) {
      await entryService.putPastEntries(unsaved);
    }
  } catch (e) {
    console.warn(e);
  }

  await navigateTo('/current');
}
</script>
