<script setup lang="ts">
import { computed } from 'vue'
import GlyphDisplay, {
  type GlyphDisplayProps,
} from '../components/GlyphDisplay.vue'
import NavBar from '../components/NavBar.vue'
import { useAppState } from '../lib/state'

const state = useAppState()

const glyphsList = computed(() => {
  if (!state.value.renderedFont) return []
  let renderedFont = state.value.renderedFont

  // glyphs are stored as `cmap` for char->glyph id mapping,
  // and then `glyphs` for glyph id -> glyph data

  let glyphs: GlyphDisplayProps[] = []
  for (let glyph of renderedFont.glyphs) {
    glyphs.push({
      chars: [],
      path: glyph.svg,
      name: undefined,
    })
  }

  for (let [char, glyphId] of Object.entries(renderedFont.cmap)) {
    let glyph = glyphs[glyphId]
    if (glyph) {
      glyph.chars.push(char)
    } else {
      console.warn(`Glyph id ${glyphId} not found for char ${char}`)
    }
  }

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
    // Use localeCompare for proper string comparison
    return a.chars[0].localeCompare(b.chars[0])
  })

  return glyphs
})
</script>

<template>
  <NavBar></NavBar>

  <div class="flex flex-row flex-wrap min-h-full p-4 gap-4">
    <GlyphDisplay
      v-for="(glyph, index) in glyphsList"
      :key="index"
      :path="glyph.path"
      :chars="glyph.chars"
      :name="glyph.name"
    ></GlyphDisplay>
  </div>
</template>
