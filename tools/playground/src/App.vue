<script setup lang="ts">
import { RouterView } from 'vue-router'
import GlobalErrorOverlay from './components/GlobalErrorOverlay.vue'
</script>

<template>
  <GlobalErrorOverlay></GlobalErrorOverlay>
  <RouterView></RouterView>
</template>
