<template>
  <div class="size-full flex-grow flex items-center justify-center place-self-center p-6">
    <div class="w-full max-w-md">
      <div class="bg-colors-primary-800/50 rounded-xl p-8 backdrop-blur-sm lg:scale-125">
        <div class="space-y-6">
          <div class="flex flex-col items-center gap-3">
            <div
                id="google-button-signin"
                data-type="standard"
                data-text="continue_with"
                :data-state="csrf"
                data-logo_alignment="center"
            />

            <div
                id="appleid-signin"
                data-color="black"
                data-border="true"
                data-type="continue"
                class="w-full h-[40px]"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { AuthService } from '~/services/auth.service';
import { invoke } from '@tauri-apps/api/core';

const { public: { apiBase, appleClientId, googleClientId } } = useRuntimeConfig();
const { $localApi } = useNuxtApp();

definePageMeta({ redirectUnautheticated: false });

const authService = new AuthService($localApi);

const { data: sessionDetails } = await useAsyncData('session-details', () => authService.getSessionDetails());
const csrf = computed(() => sessionDetails.value?.csrf_token);
const nonce = computed(() => sessionDetails.value?.nonce);

useHead({
  script: [
    // {
    //   src: 'https://accounts.google.com/gsi/client',
    //   defer: true,
    //   async: true
    // },
    {
      src: 'https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js',
      defer: true,
      async: true
    }
  ],
  meta: [
    { name: 'appleid-signin-client-id', content: appleClientId },
    { name: 'appleid-signin-scope', content: 'name' },
    { name: 'appleid-signin-redirect-uri', content: `${apiBase}/auth/apple/callback/mobile` },
    { name: 'appleid-signin-state', content: csrf },
    { name: 'appleid-signin-nonce', content: nonce },
    { name: 'appleid-signin-use-popup', content: 'false' }
  ]
});

const googleButtonInitInterval = ref<NodeJS.Timeout | null>(null);
const nonceCheckInterval = ref<NodeJS.Timeout | null>(null);

onMounted(async () => {
  nonceCheckInterval.value = setInterval(async () => {
    if (!nonce.value) {
      return;
    }

    let payload: null | string = null;
    try {
      payload = await authService.takeSession(nonce.value);
    } catch {
      /* empty */
    }

    if (!payload) {
      return;
    }

    clearInterval(nonceCheckInterval.value!);
    await navigateTo({ name: 'cb', query: { r: payload } });
  }, 500);

  const googleScriptContent = await invoke<string>('proxy_google_script');
  console.log('script', googleScriptContent);
  const googleScript = document.createElement('script');
  googleScript.type = 'text/javascript';
  googleScript.src = googleScriptContent;

  document.head.appendChild(googleScript);

  googleButtonInitInterval.value = setInterval(() => {
    // @ts-expect-error-next-line google is window type
    if (!window['google'] || !nonce.value || !csrf.value) {
      return;
    }

    clearInterval(googleButtonInitInterval.value!);
    // @ts-expect-error-next-line google is window type
    google.accounts.id.initialize({
      client_id: googleClientId,
      context: 'signin',
      ux_mode: 'redirect',
      login_uri: `${apiBase}/auth/google/callback/mobile`,
      nonce: nonce.value,
      auto_select: true,
      itp_support: true
    });

    // @ts-expect-error-next-line google is window type
    google.accounts.id.renderButton(document.getElementById('google-button-signin'), {
      type: 'standard',
      text: 'continue_with',
      state: csrf.value,
      logo_alignment: 'center'
    });

    // @ts-expect-error-next-line google is window type
    google.accounts.id.prompt();
  });
});

onUnmounted(() => {
  if (nonceCheckInterval.value) {
    clearInterval(nonceCheckInterval.value);
  }

  if (googleButtonInitInterval.value) {
    clearInterval(googleButtonInitInterval.value);
  }
});
</script>
