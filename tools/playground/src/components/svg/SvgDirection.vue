<!-- This component is supposed to be put within a SVG element. -->
<template>
  <path :d="computedPath" fill-rule="nonzero"></path>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { CubicBezier } from '../../lib/types'
import type { DraftToSvgXform } from './types'
import { drawSvgDirection } from './util'

export interface SvgDirectionProps {
  path: CubicBezier[]
  cvt: DraftToSvgXform
}
const props = defineProps<SvgDirectionProps>()

const computedPath = computed(() => {
  let res = ''
  for (let bez of props.path) {
    let dir = drawSvgDirection(bez, props.cvt)
    if (dir) {
      res += ' ' + dir
    }
  }
  return res
})
</script>
