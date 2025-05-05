<!-- This component is designed to be put within an SVG element -->
<template>
  <!-- Guide lines -->
  <g class="stroke-2 stroke-gray-300">
    <line
      v-for="(line, i) in scaledGuidelines.h"
      :key="i"
      :x1="0"
      :y1="line.pos"
      :x2="props.canvasWidth"
      :y2="line.pos"
      >{{ line.label }}</line
    >
    <line
      v-for="(line, i) in scaledGuidelines.v"
      :key="i"
      :x1="line.pos"
      :y1="0"
      :x2="line.pos"
      :y2="props.canvasHeight"
      >{{ line.label }}</line
    >
  </g>
  <!-- Labels for guide lines -->
  <g class="fill-gray-300">
    <text
      v-for="(line, i) in scaledGuidelines.h"
      :key="i"
      :x="labelMargin"
      :y="line.pos - labelMargin"
      >{{ line.label }}</text
    >
    <text
      v-for="(line, i) in scaledGuidelines.v"
      :key="i"
      :x="line.pos + labelMargin"
      :y="props.canvasHeight - labelMargin"
      :transform="`rotate(-90, ${line.pos}, ${props.canvasHeight})`"
      >{{ line.label }}</text
    >
  </g>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { SvgGuidelinesProps } from './types'
import { xformX, xformY } from './util'

const labelMargin = 8
const props = defineProps<SvgGuidelinesProps>()

const scaledGuidelines = computed(() => {
  let h = props.h.map((line) => ({
    ...line,
    pos: xformY(props.xform, line.pos),
  }))
  let v = props.v.map((line) => ({
    ...line,
    pos: xformX(props.xform, line.pos),
  }))
  return { h, v }
})
</script>
