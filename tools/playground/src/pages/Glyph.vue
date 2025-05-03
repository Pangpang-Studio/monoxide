<script setup lang="ts">
import { useRoute } from 'vue-router'
import NavBar from '../components/NavBar.vue'
import { useAppState } from '../lib/state'
import { computed, ref } from 'vue'

const route = useRoute()
const state = useAppState()

const glyphId = ref(parseInt(route.params.glyphId as string, 10) || 0)

const glyph = computed(() => {
  if (!state.value.renderedFont) return null
  let renderedFont = state.value.renderedFont

  return renderedFont.glyphs[glyphId.value]
})
</script>

<template>
  <NavBar></NavBar>
  <div
    class="flex flex-col md:flex-row min-h-screen mx-8 my-8 gap-8"
    v-if="glyph"
  >
    <!-- Glyph display area -->
    <div class="flex flex-col md:flex-basis-0 md:flex-grow-2">
      <svg
        class="width-full aspect-square border-2 border-black bg-white"
        viewBox="-0.5 -1.5 2 2"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          :d="glyph.svg"
          class="stroke-black fill-gray-300 stroke-[0.005]"
        />
        <!-- TODO: Display debug points -->
      </svg>
    </div>

    <!-- Metadata & control area -->
    <div class="flex flex-col md:flex-basis-0 md:flex-grow-1">
      <h1 class="text-2xl font-bold mt-2 mb-2">glyph #{{ glyphId }}</h1>
    </div>
  </div>
  <div v-else class="flex flex-col items-center justify-center h-screen">
    <h1 class="text-4xl font-bold">
      no glyph with the given index is available.
    </h1>
    <div>at {{ route.path }}.</div>
  </div>
</template>
