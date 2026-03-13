<script setup lang="ts">
import { computed } from 'vue'

import { useAppState } from '../lib/state'

const state = useAppState()
const appError = computed(() => state.error.value)
</script>

<template>
  <div
    v-if="appError"
    class="fixed inset-0 z-50 flex items-start justify-center bg-gray-300/60 px-4 py-6"
  >
    <div
      class="w-full max-w-3xl border-2 border-black bg-white px-4 py-3 text-black"
    >
      <div class="text-xs text-gray-600 uppercase">Application Error</div>
      <div class="mt-1 wrap-break-word whitespace-pre-wrap">
        {{ appError }}
      </div>
    </div>
  </div>
</template>
