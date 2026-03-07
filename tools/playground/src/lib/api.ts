import type { GlyphDetail, FontMetadata } from './types'

export const IS_STATIC_MODE =
  import.meta.env.VITE_PLAYGROUND_DEPLOY_MODE === 'static'

const PREBUILT_METADATA_URL =
  import.meta.env.VITE_PREBUILT_METADATA_URL ??
  `${import.meta.env.BASE_URL}assets/monoxide.ttf.meta`

const PREBUILT_TTF_URL =
  import.meta.env.VITE_PREBUILT_TTF_URL ??
  `${import.meta.env.BASE_URL}assets/monoxide.ttf`

export const FONT_FILE_URL = IS_STATIC_MODE ? PREBUILT_TTF_URL : '/api/font'

let prebuiltMetadataPromise: Promise<FontMetadata> | null = null

export async function ping() {
  if (IS_STATIC_MODE) return
  let res = await fetch('/api/ping')
  if (!res.ok) {
    throw new Error('Failed to ping API')
  }
}

export async function getGlyphDetail(id: number) {
  if (IS_STATIC_MODE) {
    let metadata = await getFontMetadata()
    let rawDetail = metadata.glyph_details[id]
    if (rawDetail === undefined)
      throw new Error(`Glyph ${id} was not found in prebuilt metadata`)
    return unwrapPrebuiltGlyphDetail(rawDetail)
  }

  let res = await fetch(`/api/glyph/${id}`)
  if (!res.ok) {
    let body = await res.text()
    throw new Error(`Failed to fetch glyph detail: ${res.statusText}; ${body}`)
  }
  let data = await res.json()
  return data as GlyphDetail
}

export async function getFontMetadata() {
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
    return data as FontMetadata
  })()

  return prebuiltMetadataPromise
}

export type RawPrebuiltGlyphDetail = { Ok: GlyphDetail } | { Err: object }

function unwrapPrebuiltGlyphDetail(raw: RawPrebuiltGlyphDetail): GlyphDetail {
  if ('Ok' in raw) return raw.Ok

  if ('Err' in raw) {
    const err = raw.Err
    if (err !== null && 'msg' in err && typeof err.msg === 'string')
      throw new Error(err.msg)
    throw new Error('Failed to fetch glyph detail: invalid error payload')
  }

  throw new Error('Failed to fetch glyph detail: invalid prebuilt payload')
}
