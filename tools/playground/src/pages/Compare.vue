<script setup lang="ts">
import { computed, ref, watchEffect } from 'vue'
import NavBar from '../components/NavBar.vue'
import { useAppState } from '../lib/state'

const textSize = ref(72)
const showMonoxide = ref(true)
const text = ref('')
const state = useAppState()
const fontFileError = computed(() => state.fontFileError.value)

/** These are the glyphs that are not found in the font */
const missingGlyphs = ref<string[]>([])
/** Each character in the type tester field to be displayed */
const glyphRuns = ref<{ ch: string; missing: boolean }[]>([])

watchEffect(() => {
  const t = text.value
  const shouldShow = showMonoxide.value

  const missing: string[] = []
  const runs: { ch: string; missing: boolean }[] = []

  const rendered = state.renderedFont.value
  for (const ch of t) {
    const missingGlyph = rendered ? !rendered.cmap.has(ch) : false
    if (shouldShow && missingGlyph) missing.push(ch)
    runs.push({ ch, missing: missingGlyph })
  }

  missingGlyphs.value = missing
  glyphRuns.value = runs
})
</script>

<template>
  <NavBar></NavBar>

  <div class="mx-4 my-4 flex flex-col">
    <div
      v-if="fontFileError"
      class="mb-3 rounded border border-red-300 bg-red-50 px-3 py-2 text-red-900"
    >
      {{ fontFileError }}
    </div>
    <div class="flex flex-row gap-6 items-baseline">
      <textarea
        v-model="text"
        type="text"
        placeholder="Type something..."
        rows="2"
        class="border-b-2 border-black grow"
      />

      <div class="flex flex-col items-baseline">
        <label for="text-size" class="font-bold">Size</label>
        <input
          v-model="textSize"
          type="number"
          id="text-size"
          min="1"
          max="192"
          class="border-b-2 border-black"
        />
      </div>
      <div class="flex flex-col gap-2 items-baseline">
        <label for="show-monoxide" class="switch">Show</label>
        <input
          type="checkbox"
          v-model="showMonoxide"
          id="show-monoxide"
          class="h-4 w-4 self-center"
        />
      </div>
    </div>

    <div class="max-w-full overflow-x-scroll">
      <div
        class="mt-4 inline-block whitespace-pre"
        :style="{ fontSize: `${textSize}px`, lineHeight: '1.2' }"
      >
        <span
          v-for="(run, index) in glyphRuns"
          :key="index"
          :class="!showMonoxide || run.missing ? 'text-gray-400' : 'text-black'"
          :style="{
            fontFamily:
              !showMonoxide || run.missing
                ? 'var(--font-mono-prioritize-1-2)'
                : `${state.fontFamily.value}, var(--font-mono-prioritize-1-2)`,
          }"
        >
          {{ run.ch }}
        </span>
      </div>
    </div>

    <div class="mt-4">
      <h2 class="text-lg font-bold">Missing Glyphs</h2>
      <ul class="list-disc pl-6">
        <li v-for="(glyph, index) in missingGlyphs" :key="index">
          {{ glyph }}
        </li>
      </ul>
      <p v-if="missingGlyphs.length === 0" class="text-gray-500">
        Crickets... :)
      </p>
    </div>
  </div>
</template>
