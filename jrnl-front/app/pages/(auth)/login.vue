<template>
  <div>

    <NuxtLink class="text-colors-text-200" @click="google">google</NuxtLink>
    <div id="appleid-signin" data-color="white" data-border="true" data-type="continue"/>
  </div>
</template>

<script lang="ts" setup>
import { UserService } from '~/services/user.service';

const { public: { apiBase, appleClientId } } = useRuntimeConfig();
const { $localApi } = useNuxtApp();

definePageMeta({ redirectUnautheticated: false });

const userService = new UserService($localApi);

const { jwt } = useAuth();
const { user } = useUser(userService);

useScript({
  src: 'https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js'
});

useHead({
  meta: [
    { name: 'appleid-signin-client-id', content: appleClientId },
    { name: 'appleid-signin-scope', content: 'name email' },
    { name: 'appleid-signin-redirect-uri', content: `${apiBase}/auth/apple/callback` },
    { name: 'appleid-signin-state', content: crypto.randomUUID() },
    { name: 'appleid-signin-use-popup', content: 'true' }
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
  let d;

  console.log(event);
  try {
    // @ts-expect-error apple sdk not typed
    d = authService.signInWithApple(event.detail.data);
  } catch (why) {
    console.error(why);
    alert('server failed');
    return;
  }

  jwt.value = d.token;
  user.value = d.user;

  try {
    await userService.updateMe({ tz: Intl.DateTimeFormat().resolvedOptions().timeZone });
  } catch (e) {
    console.warn(e);
  }

  await navigateTo('/current');
}

// eslint-disable-next-line
async function onAppleSignInFailure(event: any) {
  alert('error signing in !');
  console.log(event.detail.error);
}

const google = async () => await navigateTo(`${apiBase}/auth/google`, { external: true });
</script>