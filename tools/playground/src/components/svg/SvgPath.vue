<!-- This component is designed to be put within an SVG element -->
<template>
  <path :d="computedPath" fill-rule="nonzero"></path>
</template>

<script setup lang="ts">
import { computed, defineProps } from 'vue'
import type { CubicBezier } from '../../lib/types'
import type { DraftToSvgXform } from './types'
import { svgPenMulti } from './util'

export interface SvgPathProps {
  path: CubicBezier[]
  cvt: DraftToSvgXform
}
const props = defineProps<SvgPathProps>()

const computedPath = computed(() => {
  return svgPenMulti(props.cvt, props.path)
})
</script>
