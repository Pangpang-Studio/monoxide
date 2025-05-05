<script setup lang="ts">
import { useRoute } from 'vue-router'
import NavBar from '../components/NavBar.vue'
import { useAppState } from '../lib/state'
import { computed, ref, watchEffect, type ComputedRef, type Ref } from 'vue'
import type {
  CubicBezier,
  DebugPointKind,
  GlyphDetail,
  SerializedGlyphConstruction,
  SerializeSpiroPoint,
} from '../lib/types'
import { getGlyphDetail } from '../lib/api'
import { charList } from '../lib/util'
import SvgDebugPoint, {
  type DebugPointProps,
} from '../components/svg/SvgDebugPoint.vue'
import SvgCanvas from '../components/svg/SvgCanvas.vue'
import { SelectionMode, type SvgDebugPointInfo } from '../components/svg/types'
import { generateDebugPoints } from '../components/svg/debugPoints'

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
  if (glyphDetail.value?.overview.ch) {
    return charList([glyphDetail.value.overview.ch]) // TODO: pass more than one char here
  } else {
    return []
  }
})

// Meta & control
/// Either a selected subpart of the glyph, or overlaying another glyph
type Selected = { t: 'part'; id: number } | { t: 'overlay'; ch: string }

const selected = ref<Selected | null>(null)

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
    } else if (v.kind.t === 'boolean-add') {
      desc = 'add(' + v.kind.parents.map((p) => `%${p}`).join(', ') + ')'
    } else if (v.kind.t === 'spiro-to-bezier') {
      desc = `spiro_to_bezier(%${v.kind.parent})`
    } else {
      desc = `unknown(${v.kind})`
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
  let paths: CubicBezier[] = []
  if (glyphDetail.value) {
    let sel = selected.value
    if (sel && sel.t === 'part') {
      let part = glyphDetail.value.construction[sel.id]
      if (part.kind.t === 'cubic-bezier') {
        for (let i = 0; i < part.kind.curve.length; i++) {
          paths.push(part.kind.curve[i])
        }
      }
      if (part.result_curve) {
        for (let i = 0; i < part.result_curve.length; i++) {
          paths.push(part.result_curve[i])
        }
      }
      if (part.debug_lines) {
        for (let i = 0; i < part.debug_lines.length; i++) {
          let line = part.debug_lines[i]
          paths.push({
            start: line.from,
            segments: [
              {
                t: 'line',
                p2: line.to,
              },
            ],
            closed: false,
          })
        }
      }
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
    let part = glyphDetail.value.construction[sel.id]
    return generateDebugPoints(part)
  } else {
    return [] // TODO
  }
})
</script>

<template>
  <NavBar></NavBar>
  <div
    class="grid grid-cols-1 md:grid-cols-3 flex-grow mx-8 my-8 gap-8"
    v-if="overviewGlyph"
  >
    <!-- Glyph display area -->
    <div class="md:col-span-2">
      <!-- New canvas here -->
      <p>new canvas</p>
      <SvgCanvas
        :guidelines="{
          h: [
            { pos: 0, label: 'top' },
            { pos: 1, label: 'bottom' },
          ],
          v: [
            { pos: 0, label: 'left' },
            { pos: 0.5, label: 'right' },
          ],
        }"
        :main-path="svg"
        :selected="canvasSelectionMode"
        :debug-paths="debugPaths"
        :debug-points="debugPoints"
      ></SvgCanvas>
    </div>

    <!-- Metadata & control area -->
    <div class="md:col-span-1 overflow-scroll flex flex-col">
      <!-- ID -->
      <h1 class="text-2xl font-bold mt-2 mb-2">glyph #{{ glyphId }}</h1>
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
          class="hover:text-blue-900 cursor-pointer"
          :class="{
            'text-blue-700': selected?.t === 'part' && selected.id === step.id,
          }"
          @click="selectPart(step.id)"
        >
          <pre class="inline">%{{ step.id }} = {{ step.desc }}</pre>
        </div>
      </div>
    </div>
  </div>
  <div v-else class="flex flex-col items-center justify-center h-screen">
    <h1 class="text-4xl font-bold">
      no glyph with the given index is available.
    </h1>
    <div>at {{ route.path }}.</div>
  </div>
</template>
