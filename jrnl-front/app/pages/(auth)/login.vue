<template>
  <div>
    <div
        id="g_id_onload"
        :data-client_id="googleClientId"
        data-context="signin"
        data-ux_mode="popup"
        :data-login_uri="apiBase + '/auth/google/callback'"
        data-auto_select="true"
        data-itp_support="true"/>

    <div
        class="g_id_signin"
        data-type="standard"
        data-shape="rectangular"
        data-theme="outline"
        data-text="continue_with"
        data-size="large"
        data-logo_alignment="left"
    />

    <div id="appleid-signin" data-color="white" data-border="false" data-type="continue"/>
  </div>
</template>

<script lang="ts" setup>
import { AuthService } from '~/services/auth.service';

const { public: { apiBase, appleClientId, googleClientId } } = useRuntimeConfig();
const { $localApi } = useNuxtApp();

definePageMeta({ redirectUnautheticated: false });

const authService = new AuthService($localApi);

const { data: sessionDetails } = useAsyncData('session-details', () => authService.getSessionDetails());
const csrf = computed(() => sessionDetails.value?.csrf_token);
const nonce = computed(() => sessionDetails.value?.nonce);

useHead({
  script: [
    { src: 'https://accounts.google.com/gsi/client', defer: true, async: true },
    {
      src: 'https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js',
      defer: true,
      async: true
    }
  ],
  meta: [
    { name: 'appleid-signin-client-id', content: appleClientId },
    { name: 'appleid-signin-scope', content: 'name email' },
    { name: 'appleid-signin-redirect-uri', content: `${apiBase}/auth/apple/callback` },
    { name: 'appleid-signin-state', content: csrf },
    { name: 'appleid-signin-nonce', content: nonce },
    { name: 'appleid-signin-use-popup', content: 'false' }
  ]
});
</script>

<style>
#appleid-signin {
  width: 210px;
  height: 40px;
}
</style>