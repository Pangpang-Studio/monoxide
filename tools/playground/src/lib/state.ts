import { ref, type Ref } from 'vue'
import type { NewRenderedMsg, WSRecvMsg } from './types'

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

  ws.onmessage = (event) => {
    let data = event.data
    console.log('Received message from server:', data)
    if (typeof data === 'string') {
      let msg: WSRecvMsg = JSON.parse(data)
      if (msg.t === 'NewRendered') {
        app.renderedFont.value = msg
      } else if (msg.t === 'Error') {
        app.error.value = msg.msg
        app.renderedFont.value = null
      } else {
        console.warn('Unknown message type from server:', msg)
      }
    } else {
      console.error('Received non-string message from server:', data)
    }
  }
}

let globalState: AppState | null = null

export class AppState {
  running: Ref<Promise<void> | undefined> = ref()
  ws: WebSocket | null = null

  error: Ref<string | null> = ref(null)
  renderedFont: Ref<NewRenderedMsg | null> = ref(null)

  public started(): boolean {
    return this.running.value !== undefined
  }
}
