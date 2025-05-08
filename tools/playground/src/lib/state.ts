import { computed, ref, type ComputedRef, type Ref } from 'vue'
import type { GlyphOverview, WSRecvMsg } from './types'

export function useAppState(): AppState {
  if (globalState) {
    return globalState
  }
  let state = new AppState()
  globalState = state

  // Spawn the app running in the background
  let promise = startApp(state).catch((e) => {
    console.error('Error starting app:', e)
  })
  state.running.value = promise

  return state
}

async function startApp(app: AppState) {
  let current_href = window.location.href
  let ws_url = new URL(current_href)
  ws_url.protocol = ws_url.protocol === 'https:' ? 'wss:' : 'ws:'
  ws_url.pathname = '/api/ws'
  ws_url.search = ''
  ws_url.hash = ''

  let ws = new WebSocket(ws_url)
  app.ws = ws

  // Temporary workspace
  let pending_glyphs: GlyphOverview[] | null = null

  ws.onmessage = (event) => {
    let data = event.data
    console.log('Received message from server:', data)
    if (typeof data === 'string') {
      let msg: WSRecvMsg = JSON.parse(data)
      switch (msg.t) {
        case 'PrepareForNewEpoch':
          pending_glyphs = []
          app.receiving_new.value = true
          break

        case 'Glyph':
          if (pending_glyphs) {
            let glyph = msg as GlyphOverview
            pending_glyphs[glyph.id] = glyph
          } else {
            console.error('Received Glyph message before PrepareForNewEpoch')
          }
          break

        case 'EpochComplete':
          if (pending_glyphs) {
            let cmap = new Map<string, number>()
            for (let [k, v] of Object.entries(msg.cmap)) {
              cmap.set(k, v)
            }
            app.renderedFont.value = {
              glyphs: pending_glyphs,
              cmap,
            }
            app.receiving_new.value = false
            pending_glyphs = null
          } else {
            console.error(
              'Received EpochComplete message before PrepareForNewEpoch',
            )
          }
          break

        case 'Error':
          app.error.value = msg.msg
          app.receiving_new.value = false
          pending_glyphs = null
          break

        default:
          console.warn('Unknown message type from server:', msg)
      }
    } else {
      console.error('Received non-string message from server:', data)
    }
  }
}

export interface FontOverview {
  glyphs: GlyphOverview[]
  cmap: Map<string, number>
}

let globalState: AppState | null = null

export class AppState {
  running: Ref<Promise<void> | undefined> = ref()
  ws: WebSocket | null = null

  /** The client is currently receiving new data from server */
  receiving_new: Ref<boolean> = ref(false)
  /** The full font data received from server */
  renderedFont: Ref<FontOverview | null> = ref(null)
  /** The error received from server */
  error: Ref<string | null> = ref(null)
  /** Reverse cmap */
  revCmap: ComputedRef<Map<number, string[]>>

  constructor() {
    this.revCmap = computed(() => {
      let cmap = this.renderedFont.value?.cmap
      if (!cmap) {
        return new Map<number, string[]>()
      }
      let revCmap = new Map<number, string[]>()
      for (let [k, v] of cmap.entries()) {
        if (!revCmap.has(v)) {
          revCmap.set(v, [])
        }
        revCmap.get(v)?.push(k)
      }
      return revCmap
    })
  }

  public started(): boolean {
    return this.running.value !== undefined
  }
}
