<template>
  <div class="flex flex-col gap-2">
    <svg
      class="size-40 border-2 border-black"
      viewBox="-0.5 -1.5 2 2"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path :d="props.path" style="fill: black" />
    </svg>
    <div class="flex flex-col">
      <h3 class="font-bold" v-if="props.name">{{ props.name }}</h3>
      <div v-for="(c, i) in chars_list" :key="i" class="">
        {{ c }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, defineProps } from 'vue'

export interface GlyphDisplayProps {
  path: string
  chars: string[]
  name: string | undefined
}

const props = defineProps<GlyphDisplayProps>()
const chars_list = computed(() =>
  props.chars.map((c) => {
    let n = c.codePointAt(0)
    return `u+${n?.toString(16).padStart(4, '0')}`
  }),
)
</script>
