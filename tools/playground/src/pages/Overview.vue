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
  for (let i = 0; i < renderedFont.glyphs.length; i++) {
    let glyph = renderedFont.glyphs[i]
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
    const aHasChars = a.chars.length > 0
    const bHasChars = b.chars.length > 0

    if (aHasChars && !bHasChars) {
      return -1 // a comes first
    }
    if (!aHasChars && bHasChars) {
      return 1 // b comes first
    }
    if (!aHasChars && !bHasChars) {
      return 0 // Keep original order if both have no chars
    }
    // Both have chars, compare the first char
    return a.chars[0].codePointAt(0)! - b.chars[0].codePointAt(0)!
  })

  console.log('glyphs', glyphs)
  return glyphs
})
</script>

<template>
  <NavBar></NavBar>

  <div class="flex flex-col flex-grow p-4">
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
