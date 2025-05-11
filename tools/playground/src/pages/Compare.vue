<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import NavBar from '../components/NavBar.vue'
import { useAppState } from '../lib/state'
import type { DraftToSvgXform } from '../components/svg/types'
import { svgPenMulti } from '../components/svg/util'

const textSize = ref(72)
const text = ref('')
const state = useAppState()

/// This is the output SVG path for the text
const svgPaths = ref<string[]>([])
/// This is the viewbox for the SVG
const viewBox = ref<string>('0 0 100 100')
const svgWidth = ref(100)
const svgHeight = ref(100)
/// These are the glyphs that are not found in the font
const notFoundGlyphs = ref<string[]>([])
/// The layout function. This will never be as good as those used in the layout
/// engines, but for a simple print-and-advance test for mostly monospace glyphs
/// this should be good enough.
watchEffect(() => {
  let t = text.value
  let sz = textSize.value
  let font = state.renderedFont.value
  if (!font) {
    return
  }

  // begin of the actual layout
  let paths: string[] = []
  let hPos = 0
  let notFound: string[] = []
  for (let ch of t) {
    let glyphId = font.cmap.get(ch)
    if (glyphId === undefined) {
      notFound.push(ch)
      continue
    }
    let glyph = font.glyphs[glyphId]
    if (glyph === undefined) {
      notFound.push(ch)
      continue
    }

    let cvt: DraftToSvgXform = {
      scaleX: sz,
      scaleY: -sz,
      translateX: hPos,
      translateY: sz,
    }
    let path = svgPenMulti(cvt, glyph.outline)
    paths.push(path)

    // advance the cursor
    hPos += glyph.advance * sz
  }
  // layout end
  svgPaths.value = paths
  notFoundGlyphs.value = notFound

  // Set the viewbox to the size of the text
  let width = hPos
  let height = sz * 1.2
  viewBox.value = `0 0 ${width} ${height}`
  svgWidth.value = width
  svgHeight.value = height
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
      <svg
        class="mt-4"
        :view-box="viewBox"
        :width="svgWidth"
        :height="svgHeight"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          v-for="(path, index) in svgPaths"
          :key="index"
          :d="path"
          class="fill-black"
          fill-rule="nonzero"
        />
      </svg>
    </div>

    <div class="mt-4">
      <h2 class="text-lg font-bold">Not Found Glyphs</h2>
      <ul class="list-disc pl-6">
        <li v-for="(glyph, index) in notFoundGlyphs" :key="index">
          {{ glyph }}
        </li>
      </ul>
    </div>
  </div>
</template>
