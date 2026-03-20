<script setup lang="ts">
import { computed, ref, watchEffect, type ComputedRef, type Ref } from 'vue'
import { useRoute } from 'vue-router'

import NavBar from '../components/NavBar.vue'
import { generateDebugPoints } from '../components/svg/debugPoints'
import SvgCanvas from '../components/svg/SvgCanvas.vue'
import { SelectionMode, type SvgDebugPointInfo } from '../components/svg/types'
import { getGlyphDetail } from '../lib/api'
import { useAppState } from '../lib/state'
import type {
  CubicBezier,
  GlyphDetail,
  Point2D,
  SerializedGlyphConstruction,
} from '../lib/types'
import { charList } from '../lib/util'

const route = useRoute()
const state = useAppState()


interface Props {
  glyphId: number | string
}


let props = defineProps<Props>()


const glyphId = computed(() => {
  let id = props.glyphId
  if (typeof id === 'string') {
    id = parseInt(id)
  }
  return id
})


const overviewGlyph = computed(() => {
  let v = state.renderedFont.value
  if (!v) return null
  return v.glyphs[glyphId.value]
})


// the detail we fetch from the server
const glyphDetail: Ref<GlyphDetail | undefined> = ref()
const error: Ref<string | null> = ref(null)


watchEffect(async () => {
  // Add dependency to the state's overview
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  state.renderedFont.value

  try {
    let id = glyphId.value
    let detail = await getGlyphDetail(id)
    if (detail) {
      if (glyphId.value === id) {
        glyphDetail.value = detail
      } else {
        console.warn(
          `Glyph detail for ${id} was fetched, but the glyphId changed to ${glyphId.value}. Skipping update.`,
        )
      }
    }
  } catch (e) {
    console.error('Error fetching glyph detail:', e)
    if (e instanceof Error) {
      error.value = e.message
    } else {
      error.value = 'Unknown error'
    }
  }
})


const svg = computed(() => {
  if (glyphDetail.value) {
    return glyphDetail.value.overview.outline
  } else {
    return null
  }
})


const chars = computed(() => {
  return charList(state.revCmap.value.get(glyphId.value) ?? [])
})


// Meta & control
/// Either a selected subpart of the glyph, or overlaying another glyph
type Selected = { t: 'part'; id: number } | { t: 'overlay'; id: number }


const selected = ref<Selected | null>(null)
const selectedOverlay = ref<number | null>(null)


watchEffect(() => {
  let v = selectedOverlay.value
  if (v !== null) {
    selected.value = { t: 'overlay', id: v }
  }
})


interface ConstructionStep {
  id: number
  desc: string
  raw: SerializedGlyphConstruction
}


const constructionSteps: ComputedRef<ConstructionStep[]> = computed(() => {
  if (!glyphDetail.value) {
    return []
  }
  return glyphDetail.value.construction.map((v, i) => {
    let desc
    if (v.kind.t === 'spiro') {
      desc = `new spiro`
    } else if (v.kind.t === 'cubic-bezier') {
      desc = `new cubic_bezier`
    } else if (v.kind.t === 'stroke') {
      desc = `stroke(%${v.kind.parent}, width=${v.kind.width})`
    } else if (v.kind.t === 'transform') {
      const strOfPoint = (p: Point2D) => `[${p.x}, ${p.y}]`
      desc = `transform(%${v.kind.parent}, mov=${strOfPoint(v.kind.mov)}, mat=[${strOfPoint(v.kind.mat[0])}, ${strOfPoint(v.kind.mat[1])}])`
    } else if (v.kind.t === 'boolean-add') {
      desc = 'add(' + v.kind.parents.map((p) => `%${p}`).join(', ') + ')'
    } else if (v.kind.t === 'spiro-to-bezier') {
      desc = `spiro_to_bezier(%${v.kind.parent})`
    } else {
      desc = `unknown(${JSON.stringify(v.kind)})`
    }
    return {
      id: i,
      desc: desc,
      raw: v,
    }
  })
})


function selectPart(id: number) {
  if (selected.value?.t === 'part' && selected.value.id === id) {
    selected.value = null
    return
  }
  selected.value = { t: 'part', id: id }
}


function selectNone() {
  selected.value = null
  selectedOverlay.value = null
}


const canvasSelectionMode = computed(() => {
  let sel = selected.value
  if (sel === null) {
    return SelectionMode.None
  } else if (sel.t == 'part') {
    return SelectionMode.Part
  } else if (sel.t == 'overlay') {
    return SelectionMode.Overlay
  } else {
    console.error('Unknown selection mode', sel)
    return SelectionMode.None
  }
})


const debugPaths: ComputedRef<CubicBezier[]> = computed(() => {
  const paths: CubicBezier[] = []
  if (glyphDetail.value) {
    const sel = selected.value
    switch (sel?.t) {
      case 'part':
        debugPathsForPart(glyphDetail.value, sel.id, paths)
        break
      case 'overlay':
        break
    }
  }
  return paths
})
const debugFill: ComputedRef<CubicBezier[]> = computed(() => {
  const paths: CubicBezier[] = []
  if (glyphDetail.value) {
    const sel = selected.value
    switch (sel?.t) {
      case 'part':
        break
      case 'overlay':
        debugPathsForOverlay(sel.id, paths)
        break
    }
  }
  return paths
})


const debugPoints: ComputedRef<SvgDebugPointInfo[]> = computed(() => {
  if (!glyphDetail.value) {
    return []
  }
  let sel = selected.value
  if (!sel) {
    return []
  }
  if (sel.t === 'part') {
    let part = glyphDetail.value.construction[sel.id]!
    return generateDebugPoints(part)
  } else {
    return [] // TODO
  }
})


function debugPathsForPart(
  glyphDetail: GlyphDetail,
  id: number,
  paths: CubicBezier[],
) {
  let part = glyphDetail.construction[id]!
  if (part.kind.t === 'cubic-bezier') {
    for (const bez of part.kind.curve) paths.push(bez)
  }
  if (part.result_curve) {
    for (const bez of part.result_curve) paths.push(bez)
  }
  if (part.debug_lines) {
    for (const line of part.debug_lines) {
      paths.push({
        start: line.from,
        segments: [{ t: 'line', p2: line.to }],
        closed: false,
      })
    }
  }
}


function debugPathsForOverlay(overlayId: number, paths: CubicBezier[]) {
  let overlayGlyph = state.renderedFont.value?.glyphs[overlayId]
  if (!overlayGlyph) {
    console.warn(`Glyph with id ${overlayId} not found`)
    return
  }
  paths.push(...overlayGlyph.outline)
}


const otherGlyphsSelection = computed(() => {
  let glyphs = state.renderedFont.value?.glyphs
  if (!glyphs) {
    return []
  }
  return glyphs.map((g, i) => {
    let title: string
    if (g.name) {
      title = g.name
    } else {
      title = `Glyph #${i}`
    }
    let revChar = state.revCmap.value.get(i)
    for (let c of revChar ?? []) {
      title += ` '${c}'`
    }
    return {
      id: i,
      title: title,
    }
  })
})
</script>

<template>
  <NavBar></NavBar>
  <div
    v-if="error"
    class="mx-8 mt-6 rounded border border-red-300 bg-red-50 px-3 py-2 text-red-900"
  >
    {{ error }}
  </div>
  <div
    v-if="overviewGlyph"
    class="mx-8 my-8 grid grow grid-cols-1 gap-8 md:grid-cols-3"
  >
    <!-- Glyph display area -->
    <div class="md:col-span-2">
      <SvgCanvas
        :guidelines="glyphDetail?.guidelines ?? { h: [], v: [] }"
        :main-path="svg"
        :selected="canvasSelectionMode"
        :debug-paths="debugPaths"
        :debug-fill="debugFill"
        :debug-points="debugPoints"
      ></SvgCanvas>
    </div>

    <!-- Metadata & control area -->
    <div class="flex flex-col overflow-scroll md:col-span-1">
      <!-- ID -->
      <h1 class="mt-2 mb-2 text-2xl font-bold">glyph #{{ glyphId }}</h1>
      <!-- Characters -->
      <div class="mb-2 flex flex-col">
        <h2 class="font-bold">Characters</h2>
        <div v-for="(c, i) in chars" :key="i" class="flex flex-row gap-4">
          <div class="">{{ c.unicode }}</div>
          <pre class="inline">'{{ c.ch }}'</pre>
        </div>
      </div>
      <!-- Construction of the glyph -->
      <div class="mb-2 flex flex-col">
        <h2 class="font-bold">Construction</h2>
        <div
          v-for="(step, i) in constructionSteps"
          :key="i"
          class="cursor-pointer hover:text-blue-900"
          :class="{
            'text-blue-700': selected?.t === 'part' && selected.id === step.id,
          }"
          @click="selectPart(step.id)"
        >
          <pre class="inline">%{{ step.id }} = {{ step.desc }}</pre>
        </div>
      </div>
      <!-- Overlay another glyph -->
      <div class="mb-2 flex flex-col">
        <h2 class="font-bold">Overlay</h2>
        <div class="flex flex-row gap-2">
          <select
            v-model="selectedOverlay"
            class="mb-2 grow border-0 border-b-2 border-black py-1 hover:border-blue-700"
          >
            <option
              v-for="glyph in otherGlyphsSelection"
              :key="glyph.id"
              :value="glyph.id"
            >
              {{ glyph.title }}
            </option>
          </select>
          <button
            class="mb-2 border-2 border-black px-2 py-1 hover:border-blue-700"
            @click="selectNone"
          >
            clear
          </button>
        </div>
      </div>
    </div>
  </div>
  <div v-else class="flex h-screen flex-col items-center justify-center">
    <h1 class="text-4xl font-bold">
      no glyph with the given index is available.
    </h1>
    <div>at {{ route.path }}.</div>
  </div>
</template>
