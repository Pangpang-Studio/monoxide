import { ref, type Ref } from 'vue'
import type { NewRenderedMsg, WSRecvMsg } from './types'

export function useAppState(): Ref<AppState> {
  if (globalState) {
    return globalState
  }
  const state = ref(new AppState()) as Ref<AppState>
  globalState = state

  // Spawn the app running in the background
  let promise = startApp(state).catch((e) => {
    console.error('Error starting app:', e)
  })
  state.value.running = promise

  return state
}

async function startApp(app: Ref<AppState>) {
  let current_href = window.location.href
  let ws_url = new URL(current_href)
  ws_url.protocol = ws_url.protocol === 'https:' ? 'wss:' : 'ws:'
  ws_url.pathname = '/api/ws'
  ws_url.search = ''
  ws_url.hash = ''

  let ws = new WebSocket(ws_url)
  app.value.ws = ws

  ws.onmessage = (event) => {
    let data = event.data
    if (typeof data === 'string') {
      let msg: WSRecvMsg = JSON.parse(data)
      if (msg.t === 'NewRendered') {
        app.value.renderedFont = msg
      } else if (msg.t === 'Error') {
        app.value.error = msg.msg
      }
    } else {
      console.error('Received non-string message from server:', data)
    }
  }
}

let globalState: Ref<AppState> | null = null

export class AppState {
  running: Promise<void> | null = null
  ws: WebSocket | null = null

  error: string | null = null
  renderedFont: NewRenderedMsg | null = null

  public started(): boolean {
    return this.running !== null
  }
}
