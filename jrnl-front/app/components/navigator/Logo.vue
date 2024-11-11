<template>
  <div>
    <LazyNavigatorThemeSelector
        v-model="showThemeSelector"
        :x="lastKnownPosition?.clientX"
        :y="lastKnownPosition?.clientY"
    />
    <div
        ref="entireLogoElement"
        tabindex="-1"
        class="outline-none w-full md:h-8"
        @focusout="handleLostFocus"
    >
      <div class="flex flex-col md:flex-row justify-items-center items-center gap-2">
        <span
            :id="logoId"
            class="fixed md:static bottom-4 left-4 md:bottom-auto md:left-auto text-xl md:text-3xl !leading-normal font-semibold text-colors-primary-100/70 hover:text-colors-primary-300 md:text-colors-primary-100 transition-all duration-100 select-none touch-none cursor-pointer z-20"
            draggable="false"
            @mousedown="onPressLogo"
            @touchstart="onPressLogo"
        >
          jrnl
        </span>

        <Transition
            enter-active-class="transition-all duration-300 ease-out"
            enter-from-class="opacity-0 scale-95"
            enter-to-class="opacity-100 scale-100"
            leave-active-class="transition-all duration-200 ease-in"
            leave-from-class="opacity-100 scale-100"
            leave-to-class="opacity-0 scale-95"
        >
          <div
              v-if="isToggled || isHolding"
              class="fixed md:absolute left-4 bottom-16 md:top-12 md:bottom-auto w-[calc(100%-2rem)] md:w-64 z-10 backdrop-blur-xl bg-colors-primary-950/90 rounded-xl shadow-xl shadow-colors-primary-950/20 border border-colors-primary-800/20 overflow-hidden"
          >
            <div class="grid grid-cols-2 md:grid-cols-2 gap-2 p-2">
              <div
                  v-for="item in menuItems"
                  :key="item.path"
                  class="group w-full select-none"
                  :class="{
                  'bg-colors-primary-800/20': route.name === item.name,
                  'scale-[1.02]': hoveringName === item.name,
                }"
                  :data-name="item.name"
                  :data-path="item.path"
              >
                <div
                    class="flex items-center justify-center px-4 py-3 cursor-pointer rounded-lg transition-all duration-150 ease-out"
                    :class="{
                    'bg-colors-primary-800/40': route.name === item.name,
                    'bg-colors-primary-800/20': hoveringName === item.name,
                    'hover:bg-colors-primary-800/20 group-hover:bg-colors-primary-800/20': isToggled
                  }"
                    @click="() => goTo(item.path)"
                >
                  <span
                      class="text-base font-medium text-colors-primary-200 transition-colors duration-150"
                      :class="{
                      'text-colors-primary-50 glow': route.name === item.name,
                      'text-colors-primary-100': hoveringName === item.name,
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
    const p = getCoordsFromEvent(e);
    lastKnownPosition.value = p && {
      clientX: p.clientX,
      clientY: p.clientY
    };

    if (p) {
      if (p.clientX > window.innerWidth / 2) {
        lastKnownPosition.value!.clientX = p.clientX - (window.innerWidth / 4);
      }


      if (p.clientY > window.innerHeight / 2) {
        lastKnownPosition.value!.clientY = p.clientY - (window.innerHeight / 5);
      }
    }
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

<!--suppress CssUnresolvedCustomProperty -->
<style scoped>
.glow {
  text-shadow: hsl(var(--twc-colors-primary-500)) 0 0 16px;
}
</style>