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
    const errors: string[] = []
    for (const url of prebuiltMetadataCandidateUrls()) {
      let res = await fetch(url)
      if (!res.ok) {
        errors.push(`${url}: ${res.status} ${res.statusText}`)
        continue
      }

      try {
        return await decodeMetadataResponse(res, url)
      } catch (err) {
        const msg = err instanceof Error ? err.message : `${err}`
        errors.push(`${url}: ${msg}`)
      }
    }

    throw new Error(`Failed to fetch prebuilt metadata:\n${errors.join('\n')}`)
  })()

  return prebuiltMetadataPromise
}

const COMPRESSION_FORMATS: Record<string, CompressionFormat> = {
  ['.gz']: 'gzip',
}

function prebuiltMetadataCandidateUrls(): string[] {
  const candidates: string[] = []
  for (const ext of Object.keys(COMPRESSION_FORMATS)) {
    if (PREBUILT_METADATA_URL.endsWith(ext)) return [PREBUILT_METADATA_URL]
    candidates.push(`${PREBUILT_METADATA_URL}${ext}`)
  }

  candidates.push(PREBUILT_METADATA_URL)
  return candidates
}

async function decodeMetadataResponse(
  res: Response,
  url: string,
): Promise<FontMetadata> {
  for (const [ext, fmt] of Object.entries(COMPRESSION_FORMATS)) {
    if (url.endsWith(ext)) return await decodeCompressedMetadata(res, fmt)
  }

  const data = await res.json()
  return data as FontMetadata
}

async function decodeCompressedMetadata(
  res: Response,
  format: CompressionFormat,
): Promise<FontMetadata> {
  const bytes = new Uint8Array(await res.arrayBuffer())

  try {
    // Some hosts may transparently decompress based on content encoding.
    const plainText = new TextDecoder().decode(bytes)
    const plain = JSON.parse(plainText) as FontMetadata
    if (plain) return plain
  } catch {}

  if (typeof DecompressionStream === 'undefined')
    throw new Error('Browser does not support DecompressionStream')

  const decompressed = new Blob([bytes])
    .stream()
    .pipeThrough(new DecompressionStream(format))
  const text = await new Response(decompressed).text()
  const data = JSON.parse(text) as FontMetadata
  return data
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
