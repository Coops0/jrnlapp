<template>
  <div>
    <NuxtLink class="text-colors-text-200" @click="google">google</NuxtLink>
    <div id="appleid-signin" data-color="white" data-border="false" data-type="continue"/>
  </div>
</template>

<script lang="ts" setup>
import { AuthService } from '~/services/auth.service';
import type { AppleID } from '~/types/appleid-signin-api.type';

const { public: { apiBase, appleClientId } } = useRuntimeConfig();
const { $localApi } = useNuxtApp();

definePageMeta({ redirectUnautheticated: false });

const authService = new AuthService($localApi);

declare global {
  interface Window {
    AppleID: AppleID;
  }
}

useScript('https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js');

const { data: appleSessionDetails } = useAsyncData('apple-session-details', () => authService.getAppleSessionDetails());
const state = computed(() => appleSessionDetails.value?.state);
const code = computed(() => appleSessionDetails.value?.code);

useHead({
  meta: [
    { name: 'appleid-signin-client-id', content: appleClientId },
    { name: 'appleid-signin-scope', content: 'name email' },
    { name: 'appleid-signin-redirect-uri', content: `${apiBase}/auth/apple/callback` },
    { name: 'appleid-signin-state', content: state },
    { name: 'appleid-signin-nonce', content: code },
    { name: 'appleid-signin-use-popup', content: 'false' }
  ]
});

onMounted(() => {
  document.addEventListener('AppleIDSignInOnSuccess', onAppleSignInSuccess);
  document.addEventListener('AppleIDSignInOnFailure', onAppleSignInFailure);
});

onUnmounted(() => {
  document.removeEventListener('AppleIDSignInOnSuccess', onAppleSignInSuccess);
  document.removeEventListener('AppleIDSignInOnFailure', onAppleSignInFailure);
});

// eslint-disable-next-line
async function onAppleSignInSuccess(event: any) {
  // this shouldn't ever fire
  console.log(event);
}

// eslint-disable-next-line
async function onAppleSignInFailure(event: any) {
  // neither should this
  alert('error signing in !');
  console.log(event.detail.error);
}

const google = async () => await navigateTo(`${apiBase}/auth/google`, { external: true });
</script>

<style>
.signin-button {
  width: 210px;
  height: 40px;
}
</style>