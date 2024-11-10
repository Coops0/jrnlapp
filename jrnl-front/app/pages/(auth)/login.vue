<template>
  <div>
    <NuxtLink class="text-colors-text-200" @click="google">google</NuxtLink>
    <div
        id="g_id_onload"
        :data-client_id="googleClientId"
        data-context="signin"
        data-ux_mode="popup"
        :data-login_uri="apiBase + '/auth/google/callback'"
        :data-nonce="appleSessionDetails?.code"
        data-auto_select="true"
        data-itp_support="true"/>

    <div
        class="g_id_signin"
        data-type="standard"
        data-shape="rectangular"
        data-theme="outline"
        data-text="continue_with"
        data-size="large"
        data-logo_alignment="left"/>

    <div id="appleid-signin" data-color="white" data-border="false" data-type="continue"/>
  </div>
</template>

<script lang="ts" setup>
import { AuthService } from '~/services/auth.service';

const { public: { apiBase, appleClientId, googleClientId } } = useRuntimeConfig();
const { $localApi } = useNuxtApp();

definePageMeta({ redirectUnautheticated: false });

const authService = new AuthService($localApi);

useScript({
  key: 'appleid-auth',
  src: 'https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js'
});
useScript({
  key: 'google-signin',
  src: 'https://accounts.google.com/gsi/client',
  // async: true,
  // defer: true,
  // crossorigin: '',
  // referrerpolicy: '',
});

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
  document.addEventListener('AppleIDSignInOnFailure', onAppleSignInFailure);
});

onUnmounted(() => {
  document.removeEventListener('AppleIDSignInOnFailure', onAppleSignInFailure);
});

// eslint-disable-next-line
async function onAppleSignInFailure(event: any) {
  alert('error signing in with apple');
  console.log(event.detail.error);
}

const google = async () => await navigateTo(`${apiBase}/auth/google`, { external: true });
</script>

<style>
#appleid-signin {
  width: 210px;
  height: 40px;
}
</style>