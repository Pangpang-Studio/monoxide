<template>
  <svg
    class="w-full border-2 border-black bg-white"
    ref="svg"
    :view-box="viewbox"
    :height="height"
  >
    <SvgGuidelines
      :xform="cvt"
      :h="props.guidelines.h"
      :v="props.guidelines.v"
      :canvas-height="height"
      :canvas-width="width"
    ></SvgGuidelines>
    <!-- Main glyph -->
    <SvgPath
      v-if="props.mainPath"
      :path="props.mainPath"
      :cvt="cvt"
      class="stroke-2"
      :class="{
        'stroke-black fill-gray-200': props.selected === SelectionMode.None,
        'stroke-gray-300 fill-gray-100': props.selected === SelectionMode.Part,
        'stroke-gray-300 fill-none': props.selected === SelectionMode.Overlay,
      }"
    ></SvgPath>
    <!-- Debug paths -->
    <SvgPath
      :path="props.debugPaths"
      :cvt="cvt"
      class="stroke-2 stroke-blue-600 fill-none"
    ></SvgPath>
    <!-- Debug points -->
    <g>
      <SvgDebugPoint
        v-for="(point, i) in props.debugPoints"
        :key="i"
        :info="point"
        :cvt="cvt"
      ></SvgDebugPoint>
    </g>
  </svg>
</template>

<script setup lang="ts">
import {
  computed,
  onMounted,
  ref,
  useTemplateRef,
  watch,
  watchEffect,
  type Ref,
} from 'vue'
import {
  SelectionMode,
  type DraftToSvgXform,
  type SvgCanvasProps,
} from './types'
import SvgGuidelines from './SvgGuidelines.vue'
import SvgPath from './SvgPath.vue'
import SvgDebugPoint from './SvgDebugPoint.vue'

const svgRef = useTemplateRef('svg')

const props = defineProps<SvgCanvasProps>()

// These determine the scale of the canvas
const width = ref(0)
const height = ref(0)
const margin = computed(() => props.margin ?? 0.2)
const canvasTopleft = ref({ x: 0, y: 0 })
const canvasBottomright = ref({ x: 0, y: 0 })
const cvt: Ref<DraftToSvgXform> = ref({
  scaleX: 1,
  scaleY: 1,
  translateX: 0,
  translateY: 0,
})
const viewbox = ref('')

function updateWidth() {
  let newWidth = svgRef.value?.clientWidth || 0
  if (newWidth !== width.value) {
    width.value = newWidth // avoid triggering resize event again
  }
}
onMounted(updateWidth)

const resizeObserver = new ResizeObserver(() => {
  updateWidth()
})
onMounted(() => {
  if (svgRef.value) {
    resizeObserver.observe(svgRef.value)
  }
})
watch(svgRef, (newRef, oldRef) => {
  if (oldRef) {
    resizeObserver.unobserve(oldRef)
  }
  if (newRef) {
    resizeObserver.observe(newRef)
  }
})

watch(
  [width, margin],
  () => {
    let w = width.value
    if (w <= 0) return // dummy values
    // From here we only use width. Height should be automatically
    // calculated from the aspect ratio

    // Calculate canvas size from guidelines
    let xmin = 0
    let xmax = 0
    let ymin = 0
    let ymax = 0
    for (const g of props.guidelines.v) {
      if (g.pos < xmin) xmin = g.pos
      if (g.pos > xmax) xmax = g.pos
    }
    for (const g of props.guidelines.h) {
      if (g.pos < ymin) ymin = g.pos
      if (g.pos > ymax) ymax = g.pos
    }

    if (xmin === xmax || ymin === ymax) {
      console.warn('Invalid guidelines')
      return
    }

    // Calculate top-left and bottom-right corners of the canvas
    let canvasXmin = xmin - margin.value
    let canvasXmax = xmax + margin.value
    let canvasYmin = ymin - margin.value
    let canvasYmax = ymax + margin.value
    let canvasWidth = canvasXmax - canvasXmin
    let canvasHeight = canvasYmax - canvasYmin
    // if aspect ratio is < 1, we use 1 instead to avoid it being too tall
    if (canvasWidth < canvasHeight) {
      canvasWidth = canvasHeight
      // recalculate
      canvasXmax = canvasXmin + canvasWidth
    }
    let aspect = canvasWidth / canvasHeight

    canvasTopleft.value = { x: canvasXmin, y: canvasYmax }
    canvasBottomright.value = { x: canvasXmax, y: canvasYmin }

    // Determine the size and scale of the canvas
    let actualCanvasHeight = w / aspect
    let actualCanvasWidth = w
    height.value = actualCanvasHeight

    let scaleX = actualCanvasWidth / canvasWidth
    let scaleY = -scaleX
    console.log('scaleX', scaleX)
    console.log('scaleY', scaleY)
    cvt.value = {
      scaleX,
      scaleY,
      translateX: -canvasXmin * scaleX,
      translateY: -canvasYmax * scaleY,
    }

    let acutalCanvasXmin = canvasXmin * scaleX
    let actualCanvasYmin = canvasYmin * scaleY
    viewbox.value = `${acutalCanvasXmin} ${actualCanvasYmin} ${actualCanvasWidth} ${actualCanvasHeight}`

    console.log('Conclusion of canvas size calculation', {
      canvasWidth,
      canvasHeight,
      canvasTopleft: canvasTopleft.value,
      canvasBottomright: canvasBottomright.value,
      viewbox: viewbox.value,
      cvt: cvt.value,
    })
  },
  { immediate: true },
)
</script>
