<template>
  <div>

    <NuxtLink class="text-colors-text-200" @click="google">google</NuxtLink>
    <div @click="apple">
      <div id="appleid-signin" data-color="black" data-border="true" data-type="sign in"/>
    </div>
  </div>
</template>

<script lang="ts" setup>
const { public: { apiBase } } = useRuntimeConfig();

definePageMeta({ redirectUnautheticated: false });

useScript('https://appleid.cdn-apple.com/appleauth/static/jsapi/appleid/1/en_US/appleid.auth.js');

// useHead({
//   meta: [
//     { name: 'appleid-signin-client-id', content: 'fm.jrnl.jrnlappid' },
//     { name: 'appleid-signin-scope', content: 'name email' },
//     { name: 'appleid-signin-redirect-uri', content: `${apiBase}/auth/apple/callback` },
//     { name: 'appleid-signin-state', content: new Date().toString() },
//     // { name: 'appleid-signin-nonce', content: 'nonce' },
//     { name: 'appleid-signin-use-popup', content: 'true' }
//   ]
// });


async function appleSignin() {
  const { state, nonce } = /* todo gen state/nonce from server */ { state: 'state', nonce: 'nonce' };

  AppleID.auth.init({
    clientId: 'fm.jrnl.jrnlappid',
    scope: 'name email',
    redirectURI: `${apiBase}/auth/apple/callback`,
    state: state,
    nonce: nonce,
    usePopup: true
  });

  let data;
  try {
    data = await AppleID.auth.signIn();
    // Handle successful response.
  } catch (why) {
    console.error(why);
    alert('apple failed');
    return;
  }

  const d = authService.signInWithApple(data);

  jwt.value = d.token;
  userComposable.user.value = d.user;

  try {
    await userService.updateMe({ tz: Intl.DateTimeFormat().resolvedOptions().timeZone });
  } catch (e) {
    console.warn(e);
  }

  await navigateTo('/current');
}

const google = async () => await navigateTo(`${apiBase}/auth/google`, { external: true });
</script>