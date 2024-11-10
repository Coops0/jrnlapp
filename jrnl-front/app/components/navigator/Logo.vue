<template>
  <div ref="entireLogoElement" tabindex="-1" class="outline-none flex flex-row" @focusout="handleLostFocus">

    <NuxtLink
        :id="logoId"
        class="text-2xl font-semibold text-colors-primary-100 hover:text-colors-primary-300 transition-all duration-75 select-none touch-none"
        draggable="false"
        @mousedown="onPressLogo"
        @touchstart="onPressLogo"
    >
      jrnl
    </NuxtLink>

    <Transition
        enter-active-class="transition-all duration-200"
        enter-from-class="opacity-0 -translate-x-2"
        enter-to-class="opacity-100 translate-x-0"
        leave-active-class="transition-all duration-150"
        leave-from-class="opacity-100 translate-x-0"
        leave-to-class="opacity-0 -translate-x-2"
    >
      <div
          v-if="isToggled || isHolding"
          class="bg-colors-primary-900/20 backdrop-blur-[2px] select-none ml-4"
      >
        <div class="flex flex-row gap-1 transition-all duration-100 ease-in-out">
          <div
              v-for="item in menuItems"
              :key="item.path"
              class="select-none border-2 border-colors-accent-50"
              :class="{
                'glow': route.name === item.name,
                'scale-105 border-colors-primary-600': hoveringName === item.name,
                'hover:border-colors-primary-600 hover:scale-105': isToggled
              }"
              :style="{
                 opacity: 0.8
               }"
              :data-name="item.name"
              :data-path="item.path"
          >
            <div
                class="flex items-center justify-center px-4 cursor-pointer text-lg"
                @click="() => goTo(item.path)"
            >
              <span class="text-colors-primary-50">{{ item.name }}</span>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
const route = useRoute();

const logoId = useId();

const menuItems = [
  { name: 'current', path: '/current' },
  { name: 'groups', path: '/groups' },
  { name: 'past', path: '/past' },
  { name: 'logout', path: '/logout' }
  // todo move theme popup here
];

const entireLogoElement = ref<HTMLElement | null>(null);

const isHolding = ref(false);
const isToggled = ref(false);

const hoveringName = ref<string | null>(null);

const tryCancel = (e: TouchEvent | MouseEvent) => {
  try {
    e.preventDefault();
    e.stopPropagation();
  } catch {
    /* empty */
  }
};

function onPressLogo(e: TouchEvent | MouseEvent) {
  tryCancel(e);

  if (isToggled.value) {
    isToggled.value = false;
    isHolding.value = false;
  } else {
    isHolding.value = true;
  }
}

function handleLostFocus() {
  if (isToggled.value && entireLogoElement.value) {
    isHolding.value = false;
    isToggled.value = false;
  }
}

function onReleaseLogo() {
  if (!isHolding.value) {
    return;
  }

  if (isToggled.value) {
    isToggled.value = false;
    isHolding.value = false;
    return;
  }

  isHolding.value = false;
  isToggled.value = true;

  if (entireLogoElement.value) {
    entireLogoElement.value.focus();
  }
}

const getLinkDataSet = (target: HTMLElement) => {
  let name = null;
  let path = null;

  if (target && target.dataset) {
    name = target.dataset.name ?? null;
    path = target.dataset.path ?? null;
  }

  return { name, path };
};

const getElementsFromEvent = (e: TouchEvent | MouseEvent) => {
  let coordinateContainer: { clientX: number; clientY: number } | undefined;
  if ('touches' in e || 'changedTouches' in e) {
    coordinateContainer = (e as TouchEvent).touches?.[0] ?? (e as TouchEvent)?.changedTouches?.[0];
  } else {
    coordinateContainer = e;
  }

  if (coordinateContainer) {
    return document.elementsFromPoint(coordinateContainer.clientX, coordinateContainer.clientY);
  } else {
    return [];
  }
};

const getHoveredLinkFromMove = (e: TouchEvent | MouseEvent) => {
  return getElementsFromEvent(e)
      .map(el => getLinkDataSet(el as HTMLElement))
      .find(data => data.name) ?? null;
};

function handleDocumentHoldMove(e: MouseEvent | TouchEvent) {
  if (!isHolding.value) {
    return;
  }

  tryCancel(e);

  const heldLink = getHoveredLinkFromMove(e);
  hoveringName.value = heldLink?.name ?? null;
}

function handleDocumentHoldEnd(e: MouseEvent | TouchEvent) {
  const wasOnLogo = getElementsFromEvent(e).some(el => el.id === logoId);

  if (!wasOnLogo && !isHolding.value) {
    return;
  }

  tryCancel(e);

  if (wasOnLogo) {
    onReleaseLogo();
    return;
  }

  // if (isHolding.value)
  isHolding.value = false;

  const heldLink = getHoveredLinkFromMove(e);
  if (heldLink?.path) {
    goTo(heldLink.path);
  }
}

onMounted(() => {
  document.addEventListener('mouseup', handleDocumentHoldEnd, { passive: false });
  document.addEventListener('touchend', handleDocumentHoldEnd, { passive: false });
  document.addEventListener('mouseover', handleDocumentHoldMove, { passive: false });
  document.addEventListener('touchmove', handleDocumentHoldMove, { passive: false });
});

onUnmounted(() => {
  document.removeEventListener('mouseup', handleDocumentHoldEnd);
  document.removeEventListener('touchend', handleDocumentHoldEnd);
  document.removeEventListener('mouseover', handleDocumentHoldMove);
  document.removeEventListener('touchmove', handleDocumentHoldMove);
});

const goTo = async (path: string) => {
  isToggled.value = false;
  isHolding.value = false;

  await navigateTo(path);
};
</script>

<style scoped>
.glow {
  /*noinspection CssUnresolvedCustomProperty*/
  text-shadow: hsl(var(--twc-colors-primary-500)) 0 0 16px;
}
</style>