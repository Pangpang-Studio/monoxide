<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import NavBar from '../components/NavBar.vue'
import { useAppState } from '../lib/state'

const textSize = ref(72)
const text = ref('')
const state = useAppState()

/** These are the glyphs that are not found in the font */
const missingGlyphs = ref<string[]>([])
/** Each character in the type tester field to be displayed */
const glyphRuns = ref<{ ch: string; missing: boolean }[]>([])

watchEffect(() => {
  const t = text.value
  const rendered = state.renderedFont.value

  const missing: string[] = []
  const runs: { ch: string; missing: boolean }[] = []

  for (let i = 0; i < t.length; i++) {
    const ch = t[i]
    const missingGlyph = rendered ? !rendered.cmap.has(ch) : false
    if (missingGlyph) {
      missing.push(ch)
    }
    runs.push({ ch, missing: missingGlyph })
  }

  missingGlyphs.value = missing
  glyphRuns.value = runs
})
</script>

<template>
  <NavBar></NavBar>

  <div class="mx-4 my-4 flex flex-col">
    <div class="flex flow-row gap-6 items-baseline">
      <div class="flex flex-row gap-2 items-baseline">
        <label for="text-size">Text Size</label>
        <input
          v-model="textSize"
          type="number"
          id="text-size"
          min="1"
          max="100"
          class="py-1 border-b-2 border-black"
        />
      </div>
      <input
        v-model="text"
        type="text"
        placeholder="Enter text"
        class="py-1 border-b-2 border-black flex-grow"
      />
    </div>

    <div class="max-w-full overflow-x-scroll">
      <div
        class="mt-4 inline-block whitespace-pre"
        :style="{ fontSize: `${textSize}px`, lineHeight: '1.2' }"
      >
        <span
          v-for="(run, index) in glyphRuns"
          :key="index"
          :class="run.missing ? 'text-gray-400' : 'text-black'"
          :style="{
            fontFamily: run.missing
              ? 'var(--font-mono)'
              : `${state.fontFamily.value}, var(--font-mono)`,
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
    </div>
  </div>
</template>
