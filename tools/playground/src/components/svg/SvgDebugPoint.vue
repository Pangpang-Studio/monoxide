<!-- 
  This component is supposed to be put within a SVG element.
  See `Glyph.vue` for the usage.
-->

<script setup lang="ts">
import { computed, defineProps, type ComputedRef } from 'vue'
import type { DebugPointKind, SerializeSpiroKind } from '../../lib/types'

export type AcceptedKinds = DebugPointKind | SerializeSpiroKind
export interface DebugPointProps {
  x: number
  y: number
  kind: AcceptedKinds
  /** The forward angle, in degrees. Only used in {forward,backward}-triangle */
  forward?: number
  tag?: string
}

/** The shape of the actual point displayed */
type PointShape =
  | 'square'
  | 'circle'
  | 'forward-triangle'
  | 'backward-triangle'
  | 'diamond'
  | 'hidden'

const props = defineProps<DebugPointProps>()

const pointShape: ComputedRef<PointShape> = computed(() => {
  switch (props.kind) {
    case 'corner':
      return 'square'
    case 'g4':
    case 'g2':
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

const squareSize = 0.015
const circleSize = 0.01
const diamondSize = squareSize * Math.sqrt(2)
const forwardAngle = computed(() => props.forward ?? 0)

function strokeClass(kind: AcceptedKinds): string {
  switch (kind) {
    case 'corner':
    case 'g2':
    case 'curve':
    case 'flat':
    case 'curl':
    case 'open':
    case 'end-open':
      return 'stroke-blue-700'
    case 'g4':
      return 'stroke-blue-800'
    case 'anchor':
    case 'misc':
    case 'control':
    case 'handle':
    case 'hidden':
      return 'stroke-gray-500'
  }
}

const classes = computed(() => {
  return ['stroke-2', 'fill-white', strokeClass(props.kind)]
})
</script>

<template>
  <circle
    v-if="pointShape === 'circle'"
    :cx="props.x"
    :cy="props.y"
    :r="circleSize"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></circle>
  <rect
    v-else-if="pointShape === 'square'"
    :x="props.x - squareSize / 2"
    :y="props.y - squareSize / 2"
    :width="squareSize"
    :height="squareSize"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></rect>
  <path
    v-else-if="pointShape === 'diamond'"
    :d="`M 0 ${-diamondSize} L ${diamondSize} 0 L 0 ${diamondSize} L ${-diamondSize} 0 Z`"
    :transform="`translate(${props.x}, ${props.y}) rotate(45)`"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></path>
  <path
    v-else-if="pointShape === 'forward-triangle'"
    :d="`M 0 ${-circleSize / 2} L ${circleSize} ${circleSize} L ${-circleSize} ${circleSize} Z`"
    :transform="`translate(${props.x}, ${props.y}) rotate(${forwardAngle})`"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></path>
  <path
    v-else-if="pointShape === 'backward-triangle'"
    :d="`M 0 ${-circleSize / 2} L ${circleSize} ${circleSize} L ${-circleSize} ${circleSize} Z`"
    :transform="`translate(${props.x}, ${props.y}) rotate(${forwardAngle + 180})`"
    :class="classes"
    vector-effect="non-scaling-stroke"
  ></path>
  <template v-else-if="pointShape === 'hidden'">
    <!-- no-op for hidden points -->
  </template>

  <!-- tag -->
  <text
    v-if="props.tag"
    :x="props.x + squareSize * 1.5"
    :y="props.y - squareSize / 2"
  ></text>
</template>
