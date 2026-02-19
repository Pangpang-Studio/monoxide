<!-- 
  This component is supposed to be put within a SVG element.
  See `Glyph.vue` for the usage.
-->

<script setup lang="ts">
import { computed, type ComputedRef } from 'vue'
import type { AcceptedKinds, DraftToSvgXform, SvgDebugPointInfo } from './types'
import { xformPoint } from './util'

export interface DebugPointProps {
  info: SvgDebugPointInfo
  cvt: DraftToSvgXform
}

/** The shape of the actual point displayed */
type PointShape =
  | 'square'
  | 'rounded-square'
  | 'circle'
  | 'forward-triangle'
  | 'backward-triangle'
  | 'diamond'
  | 'hidden'

const props = defineProps<DebugPointProps>()

const pointShape: ComputedRef<PointShape> = computed(() => {
  switch (props.info.kind) {
    case 'corner':
      return 'square'
    case 'g2':
      return 'rounded-square'
    case 'g4':
    case 'control':
    case 'curve':
    case 'handle':
      return 'circle'
    case 'flat':
    case 'end-open':
      return 'backward-triangle'
    case 'curl':
    case 'open':
      return 'forward-triangle'
    case 'anchor':
    case 'misc':
      return 'diamond'
    case 'hidden':
      return 'hidden'
  }
})

const squareSize = 8
const circleSize = squareSize
const diamondSize = squareSize * Math.sqrt(2)
const forwardAngle = computed(() => props.info.forward ?? 0)

function strokeClass(kind: AcceptedKinds): string {
  switch (kind) {
    case 'corner':
    case 'g2':
    case 'g4':
    case 'curve':
    case 'flat':
    case 'curl':
    case 'open':
    case 'end-open':
      return 'stroke-blue-700'
    case 'anchor':
    case 'misc':
    case 'control':
    case 'handle':
    case 'hidden':
      return 'stroke-gray-500'
  }
}

const classes = computed(() => {
  return ['stroke-2', 'fill-white', strokeClass(props.info.kind)]
})

const coords = computed(() => xformPoint(props.cvt, props.info))
</script>

<template>
  <circle
    v-if="pointShape === 'circle'"
    :cx="coords.x"
    :cy="coords.y"
    :r="circleSize / 2"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></circle>
  <rect
    v-else-if="pointShape === 'square'"
    :x="coords.x - squareSize / 2"
    :y="coords.y - squareSize / 2"
    :width="squareSize"
    :height="squareSize"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></rect>
  <rect
    v-else-if="pointShape === 'rounded-square'"
    :x="coords.x - squareSize / 2"
    :y="coords.y - squareSize / 2"
    :width="squareSize"
    :height="squareSize"
    :rx="squareSize / 4"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></rect>
  <path
    v-else-if="pointShape === 'diamond'"
    :d="`M 0 ${-diamondSize} L ${diamondSize} 0 L 0 ${diamondSize} L ${-diamondSize} 0 Z`"
    :transform="`translate(${coords.x}, ${coords.y}) rotate(45)`"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></path>
  <path
    v-else-if="pointShape === 'forward-triangle'"
    :d="`M 0 ${-circleSize / 4} L ${circleSize / 2} ${circleSize / 2} L ${-circleSize / 2} ${circleSize / 2} Z`"
    :transform="`translate(${coords.x}, ${coords.y}) rotate(${forwardAngle})`"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></path>
  <path
    v-else-if="pointShape === 'backward-triangle'"
    :d="`M 0 ${-circleSize / 4} L ${circleSize / 2} ${circleSize / 2} L ${-circleSize / 2} ${circleSize / 2} Z`"
    :transform="`translate(${coords.x}, ${coords.y}) rotate(${forwardAngle + 180})`"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></path>
  <template v-else-if="pointShape === 'hidden'">
    <!-- no-op for hidden points -->
  </template>

  <!-- tag -->
  <text
    v-if="props.info.tag"
    :x="coords.x + squareSize * 1.5"
    :y="coords.y - squareSize / 2"
  ></text>
</template>
