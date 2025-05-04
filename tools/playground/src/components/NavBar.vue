<template>
  <div class="flex flex-row px-1">
    <div aria-label="logo" class="px-3 py-2 font-bold">monoxide</div>
    <template
      v-for="route in computedRoutes"
      :key="route.name"
      :to="route.path"
    >
      <RouterLink
        v-if="route.path"
        :to="route.path"
        class="px-3 py-2 hover:bg-gray-100"
        :class="{ 'text-blue-600': route.isActive }"
      >
        {{ route.name }}
      </RouterLink>
      <div
        v-else
        class="px-3 py-2"
        :class="{ 'text-blue-600': route.isActive }"
      >
        {{ route.name }}
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'
import { onBeforeRouteUpdate, useRoute, type RouteLocation } from 'vue-router'
import { RouterLink } from 'vue-router'

interface NavRoute {
  matcher: RegExp
  name: string
  path: string | null
}

const routes: NavRoute[] = [
  { matcher: /\/$/, name: 'home', path: '/' },
  { matcher: /glyph/, name: 'glyph', path: null },
  { matcher: /compare/, name: 'compare', path: '/compare' },
]

interface ComputedRoute extends NavRoute {
  isActive: boolean
}

const route = useRoute()
const computedRoutes: Ref<ComputedRoute[]> = ref(calcRoutes(route))

function calcRoutes(route: RouteLocation) {
  return routes.map((thisRoute) => {
    let isActive = thisRoute.matcher.test(route.path)
    return {
      ...thisRoute,
      isActive,
    }
  })
}

onBeforeRouteUpdate((_from, to) => {
  computedRoutes.value = calcRoutes(to)
})
</script>
