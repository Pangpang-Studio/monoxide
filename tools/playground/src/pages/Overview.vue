<script setup lang="ts">
import { computed } from 'vue'
import GlyphDisplay, {
  type GlyphDisplayProps,
} from '../components/GlyphDisplay.vue'
import NavBar from '../components/NavBar.vue'
import { useAppState } from '../lib/state'

const state = useAppState()

const glyphsList = computed(() => {
  let renderedFont = state.renderedFont.value
  if (!renderedFont) return []

  // glyphs are stored as `cmap` for char->glyph id mapping,
  // and then `glyphs` for glyph id -> glyph data

  let glyphs: GlyphDisplayProps[] = []
  for (const glyph of renderedFont.glyphs) {
    glyphs.push({
      overview: glyph,
      chars: [],
      name: undefined,
    })
  }

  for (let [char, glyphId] of renderedFont.cmap.entries()) {
    let glyph = glyphs[glyphId]
    if (glyph) {
      glyph.chars.push(char)
    } else {
      console.warn(`Glyph id ${glyphId} not found for char ${char}`)
    }
  }

  // Sort glyphs based on the first char in chars array
  glyphs.sort((a, b) => {
    const aHead = a.chars[0]
    const bHead = b.chars[0]

    if (aHead === undefined) {
      return bHead === undefined
        ? 0 // Keep original order if both have no chars
        : 1 // b comes first
    }
    return bHead === undefined
      ? -1 // a comes first
      : aHead.codePointAt(0)! - bHead.codePointAt(0)! // Compare the first char
  })

  console.log('glyphs', glyphs)
  return glyphs
})
</script>

<template>
  <NavBar></NavBar>

  <div class="flex flex-col grow p-4">
    <h1 class="text-2xl font-bold mt-2 mb-2">glyphs list</h1>
    <div class="flex flex-row flex-wrap -mx-2 items-start">
      <GlyphDisplay
        v-for="(glyph, index) in glyphsList"
        :key="index"
        v-bind="glyph"
      ></GlyphDisplay>
    </div>
  </div>
</template>
