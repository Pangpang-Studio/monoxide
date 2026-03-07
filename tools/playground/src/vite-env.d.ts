/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_PLAYGROUND_DEPLOY_MODE?: 'dynamic' | 'static'
  readonly VITE_PREBUILT_METADATA_URL?: string
  readonly VITE_PREBUILT_TTF_URL?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
