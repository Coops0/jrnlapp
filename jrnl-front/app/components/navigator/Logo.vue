<template>
  <div>
    <LazyNavigatorThemeSelector
        v-model="showThemeSelector"
        :x="lastKnownPosition?.clientX"
        :y="lastKnownPosition?.clientY"
    />
    <div ref="entireLogoElement" tabindex="-1" class="outline-none w-full md:h-8" @focusout="handleLostFocus">
      <div class="flex flex-col md:flex-row justify-items-center items-center gap-2">
      <span
          :id="logoId"
          class="text-3xl !leading-normal font-semibold text-colors-primary-100 hover:text-colors-primary-300 transition-all duration-100 select-none touch-none cursor-pointer"
          draggable="false"
          @mousedown="onPressLogo"
          @touchstart="onPressLogo"
      >
        jrnl
      </span>

        <Transition
            enter-active-class="transition-all duration-200"
            enter-from-class="opacity-0 md:-translate-x-2 -translate-y-2 md:translate-y-0"
            enter-to-class="opacity-100 translate-x-0 translate-y-0"
            leave-active-class="transition-all duration-150"
            leave-from-class="opacity-100 translate-x-0 translate-y-0"
            leave-to-class="opacity-0 md:-translate-x-2 -translate-y-2 md:translate-y-0"
        >
          <div
              v-if="isToggled || isHolding"
              class="w-full max-h-full flex-grow absolute mt-12 md:mt-0 md:relative z-40 backdrop-blur-xl bg-colors-primary-950/40 rounded-lg overflow-hidden"
          >
            <div class="flex flex-col md:flex-row md:justify-evenly md:justify-items-stretch w-full">
              <div
                  v-for="item in menuItems"
                  :key="item.path"
                  class="group w-full md:flex-grow select-none px-1 py-0.5"
                  :class="{
                  'bg-colors-primary-800/20': route.name === item.name,
                  'scale-[1.02]': hoveringName === item.name,
                }"
                  :data-name="item.name"
                  :data-path="item.path"
              >
                <div
                    class="flex items-center justify-center px-6 py-3 md:py-2 cursor-pointer rounded-md transition-all duration-150 ease-out"
                    :class="{
                    'bg-colors-primary-800/40': route.name === item.name,
                    'bg-colors-primary-800/20': hoveringName === item.name,
                    'hover:bg-colors-primary-800/20 group-hover:bg-colors-primary-800/20': isToggled
                  }"
                    @click="() => goTo(item.path)"
                >
                <span
                    class="text-lg md:text-base text-colors-primary-200 transition-colors duration-150"
                    :class="{
                      'text-colors-primary-50 glow': route.name === item.name,
                      'text-color-primary-100': hoveringName === item.name,
                      'group-hover:text-colors-primary-50': isToggled
                    }"
                >
                  {{ item.name }}
                </span>
                </div>
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const route = useRoute();

const logoId = useId();

const menuItems = [
  { name: 'current', path: '/current' },
  { name: 'groups', path: '/groups' },
  { name: 'past', path: '/past' },
  { name: 'theme', path: '/theme' },
  { name: 'logout', path: '/logout' }
];

const showThemeSelector = ref(false);
const lastKnownPosition = ref<{ clientX: number; clientY: number } | null>(null);

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

  if (target?.dataset) {
    name = target.dataset.name ?? null;
    path = target.dataset.path ?? null;
  }

  return { name, path };
};

const getCoordsFromEvent = (e: TouchEvent | MouseEvent): { clientX: number; clientY: number } | null => {
  if ('touches' in e || 'changedTouches' in e) {
    return (e as TouchEvent).touches?.[0] ?? (e as TouchEvent)?.changedTouches?.[0] ?? null;
  } else {
    return e;
  }
};

const getElementsFromEvent = (e: TouchEvent | MouseEvent) => {
  const coordinateContainer = getCoordsFromEvent(e);
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
  if (!showThemeSelector.value) {
    // for theme selector to pop up at mouse
    lastKnownPosition.value = getCoordsFromEvent(e);
  }

  if (!isHolding.value) {
    return;
  }

  tryCancel(e);

  const heldLink = getHoveredLinkFromMove(e);
  hoveringName.value = heldLink?.name ?? null;
}

function handleDocumentHoldEnd(e: MouseEvent | TouchEvent) {
  const wasOnLogo = getElementsFromEvent(e)
      .some(el => el.id === logoId);

  if (!wasOnLogo && !isHolding.value) {
    return;
  }

  tryCancel(e);

  const link = getElementsFromEvent(e)
      .map(el => getLinkDataSet(el as HTMLElement))
      .find(el => !!el.name);

  if (wasOnLogo) {
    // current is first & overlaps
    if (link?.name === 'current' && isHolding.value) {
      goTo(link.path!);
    } else {
      onReleaseLogo();
    }

    return;
  }

  if (!isHolding.value) {
    return;
  }

  if (link?.path) {
    goTo(link.path);
  }

  isHolding.value = false;
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
  hoveringName.value = null;

  if (path === '/theme') {
    showThemeSelector.value = true;
  } else {
    await navigateTo(path);
  }
};
</script>

<style scoped>
.glow {
  /*noinspection CssUnresolvedCustomProperty*/
  text-shadow: hsl(var(--twc-colors-primary-500)) 0 0 16px;
}
</style>