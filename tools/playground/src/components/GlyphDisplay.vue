<template>
  <RouterLink
    :to="`/glyph/${props.overview.id}`"
    class="flex flex-col gap-2 p-2 hover:bg-gray-100"
  >
    <svg
      class="size-40 border-2 border-black bg-white"
      viewBox="-0.5 -1.5 2 2"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path :d="drawnPath" style="fill: black" fill-rule="nonzero" />
    </svg>
    <div class="flex flex-col">
      <h3 class="font-bold" v-if="props.name">{{ props.name }}</h3>
      <div>#{{ props.overview.id }}</div>
      <div v-for="(c, i) in chars_list" :key="i" class="">
        {{ c.unicode }}
        <pre class="inline">'{{ props.chars[i] }}'</pre>
      </div>
      <div v-if="!chars_list">(no chars maps to this glyph)</div>
    </div>
  </RouterLink>
</template>

<script setup lang="ts">
import { computed, defineProps } from 'vue'
import { charList } from '../lib/util'
import type { GlyphOverview } from '../lib/types'
import { svgPenMulti } from './svg/util'
import type { DraftToSvgXform } from './svg/types'

export interface GlyphDisplayProps {
  overview: GlyphOverview
  chars: string[]
  name: string | undefined
}

const props = defineProps<GlyphDisplayProps>()
const chars_list = computed(() => charList(props.chars))

const drawnPath = computed(() => {
  let overview = props.overview

  // We would like the glyph to stay centered here, so we calculate it from the
  // glyph's advance (i.e. width). Center is 0.5
  let advance = overview.advance
  let x_offset = 0.5 - advance / 2

  let cvt: DraftToSvgXform = {
    scaleX: 1,
    scaleY: -1,
    translateX: x_offset,
    translateY: 0,
  }
  let path = svgPenMulti(cvt, overview.outline)
  return path
})
</script>
