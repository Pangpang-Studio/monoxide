<template>
  <div class="flex flex-col items-center justify-center h-screen">
    <h1 class="text-4xl font-bold">404</h1>
    <div>
      at {{ route.path }}.
      <RouterLink to="/" class="text-blue-600">return home</RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { useRoute } from 'vue-router'

let route = useRoute()
</script>
