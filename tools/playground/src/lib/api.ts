import type { GlyphDetail, PrebuiltMetadata } from './types'

const STATIC_MODE = import.meta.env.VITE_PLAYGROUND_DEPLOY_MODE === 'static'
const PREBUILT_METADATA_URL =
  import.meta.env.VITE_PREBUILT_METADATA_URL ??
  `${import.meta.env.BASE_URL}assets/monoxide.ttf.meta`
const PREBUILT_TTF_URL =
  import.meta.env.VITE_PREBUILT_TTF_URL ??
  `${import.meta.env.BASE_URL}assets/monoxide.ttf`

let prebuiltMetadataPromise: Promise<PrebuiltMetadata> | null = null

export function isStaticDeployMode() {
  return STATIC_MODE
}

export function fontFileUrl() {
  return STATIC_MODE ? PREBUILT_TTF_URL : '/api/font'
}

export async function ping() {
  if (STATIC_MODE) {
    return
  }
  let res = await fetch('/api/ping')
  if (!res.ok) {
    throw new Error('Failed to ping API')
  }
}

export async function getGlyphDetail(id: number) {
  if (STATIC_MODE) {
    let metadata = await getPrebuiltMetadata()
    let rawDetail = metadata.glyph_details[id]
    if (rawDetail === undefined) {
      throw new Error(`Glyph ${id} was not found in prebuilt metadata`)
    }

    let detail = decodePrebuiltGlyphDetail(rawDetail)
    if (detail instanceof Error) {
      throw detail
    }
    return detail
  }

  let res = await fetch(`/api/glyph/${id}`)
  if (!res.ok) {
    let body = await res.text()
    throw new Error(`Failed to fetch glyph detail: ${res.statusText}; ${body}`)
  }
  let data = await res.json()
  return data as GlyphDetail
}

export async function getPrebuiltMetadata() {
  if (prebuiltMetadataPromise) {
    return prebuiltMetadataPromise
  }

  prebuiltMetadataPromise = (async () => {
    let res = await fetch(PREBUILT_METADATA_URL)
    if (!res.ok) {
      let body = await res.text()
      throw new Error(
        `Failed to fetch prebuilt metadata: ${res.statusText}; ${body}`,
      )
    }
    let data = await res.json()
    return data as PrebuiltMetadata
  })()

  return prebuiltMetadataPromise
}

function decodePrebuiltGlyphDetail(raw: unknown): GlyphDetail | Error {
  if (isObject(raw) && 'Ok' in raw) {
    return (raw as { Ok: GlyphDetail }).Ok
  }

  if (isObject(raw) && 'Err' in raw) {
    const err = (raw as { Err: unknown }).Err
    if (isObject(err) && typeof err.msg === 'string') {
      return new Error(err.msg)
    }
    return new Error('Failed to fetch glyph detail: invalid error payload')
  }

  return new Error('Failed to fetch glyph detail: invalid prebuilt payload')
}

function isObject(v: unknown): v is Record<string, unknown> {
  return typeof v === 'object' && v !== null
}
