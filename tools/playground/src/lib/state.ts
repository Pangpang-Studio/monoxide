import { ref, type Ref } from 'vue'

export function useAppState(): Ref<AppState> {
  if (globalState) {
    return globalState
  }
  const state = ref(new AppState()) as Ref<AppState>
  globalState = state

  // Spawn the app running in the background
  startApp(state).catch((e) => {
    console.error('Error starting app:', e)
  })

  return state
}

async function startApp(app: Ref<AppState>) {}

let globalState: Ref<AppState> | null = null

export class AppState {
  running: Promise<void> | null = null

  public started(): boolean {
    return this.running !== null
  }
}
