<template>
  <div class="size-full flex-grow flex items-center justify-center place-self-center p-6">
    <div class="w-full max-w-md">
      <div class="bg-colors-primary-800/50 rounded-xl p-8 backdrop-blur-sm lg:scale-125">
        <div class="space-y-6">
          <div class="flex flex-col items-center gap-3">
            <div id="google-button-signin"/>
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

const { public: { appleClientId, googleClientId } } = useRuntimeConfig();
const { $localApi } = useNuxtApp();

const authService = new AuthService($localApi);
const userService = new UserService($localApi);

const { jwt } = await useAuth();
const { user, hasRefreshedRemotely } = await useUser(userService);

const { data: sessionDetails } = await useAsyncData('session-details', () => authService.getSessionDetails());

const error = ref<string | null>(null);

// useHead({
//   script: [
//     {
//       src: 'https://accounts.google.com/gsi/client',
//       async: true
//     }
//   ]
// });

const scriptCheckInterval = ref<NodeJS.Timeout | null>(null);


// @ts-expect-error google & AppleID are defined from google & apple scripts
const isReady = () => typeof google !== 'undefined' && typeof AppleID !== 'undefined';

onMounted(() => {
  if (isReady()) {
    if (scriptCheckInterval.value) {
      clearInterval(scriptCheckInterval.value);
      scriptCheckInterval.value = null;
    }
    initGoogle();
  } else if (!scriptCheckInterval.value) {
    scriptCheckInterval.value = setInterval(() => {
      if (isReady()) {
        clearInterval(scriptCheckInterval.value!);
        initGoogle();
      }
    }, 250);
  }
});

function initGoogle() {
  if (!sessionDetails.value) {
    throw createError({
      statusCode: 500,
      message: 'failed to generate session details'
    });
  }

  const { csrf_token: state, nonce } = sessionDetails.value;

  // @ts-expect-error google is defined from google script
  // noinspection JSUnusedGlobalSymbols
  google.accounts.id.initialize({
    client_id: googleClientId,
    context: 'use',
    ux_mode: 'popup',
    nonce,
    auto_select: true,
    itp_support: true,
    callback: async (googlePayload: unknown) => {
      try {
        const response = await authService.loginWithGoogle(googlePayload);
        await handleServerResponse(response);
      } catch (e) {
        console.error(e);
        error.value = 'failed to login with google';
      }
    },
  });

  // @ts-expect-error google is defined from google script
  google.accounts.id.renderButton(document.getElementById('google-button-signin'), {
    type: 'standard',
    text: 'continue_with',
    state,
    logo_alignment: 'center'
  });

  try {
    // @ts-expect-error google is defined from google script
    google.accounts.id.prompt();
  } catch {
    /* empty */
  }
}

async function startAppleLogin() {
  const response = await get_apple_id_credential({
    scope: ['fullName'],
    nonce: sessionDetails.value!.nonce,
    state: sessionDetails.value!.csrf_token
  });

  console.log(response);
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
