<template>
  <RouterLink :to="`/glyph/${props.id}`">
    <div class="flex flex-col gap-2 p-2 hover:bg-gray-100">
      <svg
        class="size-40 border-2 border-black bg-white"
        viewBox="-0.5 -1.5 2 2"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path :d="props.path" style="fill: black" fill-rule="nonzero" />
      </svg>
      <div class="flex flex-col">
        <h3 class="font-bold" v-if="props.name">{{ props.name }}</h3>
        <div v-for="(c, i) in chars_list" :key="i" class="">
          {{ c.unicode }}
          <pre class="inline">'{{ props.chars[i] }}'</pre>
        </div>
        <div v-if="!chars_list">(no chars maps to this glyph)</div>
      </div>
    </div>
  </RouterLink>
</template>

<script setup lang="ts">
import { computed, defineProps } from 'vue'
import { charList } from '../lib/util'

export interface GlyphDisplayProps {
  id: number
  path: string
  chars: string[]
  name: string | undefined
}

const props = defineProps<GlyphDisplayProps>()
const chars_list = computed(() => charList(props.chars))
</script>
