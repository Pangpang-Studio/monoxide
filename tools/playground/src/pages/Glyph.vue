<script setup lang="ts">
import { useRoute } from 'vue-router'
import NavBar from '../components/NavBar.vue'
import { useAppState } from '../lib/state'
import { computed, ref, watch, watchEffect, type Ref } from 'vue'
import type { GlyphDetail } from '../lib/types'
import { getGlyphDetail } from '../lib/api'

const route = useRoute()
const state = useAppState()

interface Props {
  glyphId: number | string
}

let props = defineProps<Props>()

const glyphId = computed(() => {
  let id = props.glyphId
  if (typeof id === 'string') {
    id = parseInt(id)
  }
  return id
})

const overviewGlyph = computed(() => {
  if (!state.value.renderedFont) return null
  let renderedFont = state.value.renderedFont

  return renderedFont.glyphs[glyphId.value]
})

// the detail we fetch from the server
const glyphDetail: Ref<GlyphDetail | undefined> = ref()
const error: Ref<string | null> = ref(null)

async function updateDetails(newGlyphId: number) {
  try {
    if (!state.value.renderedFont) return
    let id = newGlyphId
    let detail = await getGlyphDetail(id)
    if (detail) {
      if (glyphId.value === id) {
        glyphDetail.value = detail
      } else {
        console.warn(
          `Glyph detail for ${id} was fetched, but the glyphId changed to ${glyphId.value}. Skipping update.`,
        )
      }
    }
  } catch (e) {
    console.error('Error fetching glyph detail:', e)
    if (e instanceof Error) {
      error.value = e.message
    } else {
      error.value = 'Unknown error'
    }
  }
}
watchEffect(() => {
  if (glyphId.value !== undefined) {
    updateDetails(glyphId.value)
  }
})
</script>

<template>
  <NavBar></NavBar>
  <div
    class="grid grid-cols-1 md:grid-cols-3 min-h-screen mx-8 my-8 gap-8"
    v-if="overviewGlyph"
  >
    <!-- Glyph display area -->
    <div class="md:col-span-2">
      <svg
        class="w-full aspect-square border-2 border-black bg-white"
        viewBox="-0.5 -1.5 2 2"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          :d="overviewGlyph.svg"
          class="stroke-black fill-gray-300 stroke-[0.005]"
          fill-rule="nonzero"
        />
        <!-- TODO: Display debug points -->
      </svg>
    </div>

    <!-- Metadata & control area -->
    <div class="md:col-span-1 overflow-scroll">
      <h1 class="text-2xl font-bold mt-2 mb-2">glyph #{{ glyphId }}</h1>
      <pre>{{ glyphDetail }}</pre>
    </div>
  </div>
  <div v-else class="flex flex-col items-center justify-center h-screen">
    <h1 class="text-4xl font-bold">
      no glyph with the given index is available.
    </h1>
    <div>at {{ route.path }}.</div>
  </div>
</template>
