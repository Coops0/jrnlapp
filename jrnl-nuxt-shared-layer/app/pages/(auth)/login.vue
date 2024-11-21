<template>
  <div class="size-full flex-grow flex items-center justify-center place-self-center p-6">
    <div class="w-full max-w-md">
      <div class="bg-colors-primary-800/50 rounded-xl p-8 backdrop-blur-sm lg:scale-125">
        <div class="space-y-6">
          <div class="flex flex-col items-center gap-3">
            <div id="google-button-signin"/>

            <div
                id="appleid-signin"
                data-color="black"
                data-border="true"
                data-type="continue"
                class="w-full h-[40px]"
            />
          </div>

          <ErrorDisplay v-if="error" :error="error"/>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { AuthService, type ServerResponse } from '~/services/auth.service';
import { UserService } from '~/services/user.service';
import { watchErrorAndThrow } from '~/util/watch-error-and-throw.util';

const { public: { appleClientId, googleClientId } } = useRuntimeConfig();
const { $localApi } = useNuxtApp();

definePageMeta({ redirectUnautheticated: false });

const authService = new AuthService($localApi);
const userService = new UserService($localApi);

const { jwt } = useAuth();
const { user, hasRefreshedRemotely } = useUser(userService);

const {
  data: sessionDetails,
  error: sessionError
} = useAsyncData('session-details', () => authService.getSessionDetails());
watchErrorAndThrow(sessionError);

const error = ref<string | null>(null);

useHead({
  script: [
    {
      src: 'https://accounts.google.com/gsi/client',
      async: true
    },
    {
      src: 'https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js',
      async: true
    }
  ]
});

const scriptCheckInterval = ref<NodeJS.Timeout | null>(null);


// @ts-expect-error google & AppleID are defined from google & apple scripts
const isReady = () => typeof google !== 'undefined' && typeof AppleID !== 'undefined';

onMounted(() => {
  if (isReady()) {
    if (scriptCheckInterval.value) {
      clearInterval(scriptCheckInterval.value);
      scriptCheckInterval.value = null;
    }
    initProviders();
  } else if (!scriptCheckInterval.value) {
    scriptCheckInterval.value = setInterval(() => {
      if (isReady()) {
        clearInterval(scriptCheckInterval.value!);
        initProviders();
      }
    }, 250);
  }
});

function initProviders() {
  if (!sessionDetails.value) {
    throw createError({
      statusCode: 500,
      message: 'failed to generate session details'
    });
  }

  const { csrf_token: state, nonce } = sessionDetails.value;

  // @ts-expect-error AppleID is defined from apple script
  AppleID.auth.init({
    clientId: appleClientId,
    scope: 'name',
    state,
    nonce,
    usePopup: true,
    // redirectURI: `${apiBase}/auth/apple`
    // redirectURI: 'https://my.jrnl.fm/auth/apple'
    redirectURI: window.location.href
  });

  document.addEventListener('AppleIDSignInOnSuccess', async event => {
    try {
      // @ts-expect-error event is not typed
      const response = await authService.loginWithApple(event?.detail);
      await handleServerResponse(response);
    } catch (e) {
      console.error(e);
      error.value = 'failed to login with apple';
    }
  });

  document.addEventListener('AppleIDSignInOnFailure', event => {
    // @ts-expect-error event is not typed
    if (event.detail.error === 'popup_closed_by_user') {
      error.value = 'the popup was closed';
    } else {
      // @ts-expect-error event is not typed
      error.value = event?.detail?.error?.replaceAll('_', ' ');
    }
  });

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
