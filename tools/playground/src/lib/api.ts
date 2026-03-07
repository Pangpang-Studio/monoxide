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

function prebuiltMetadataCandidateUrls(): string[] {
  if (
    PREBUILT_METADATA_URL.endsWith('.zst') ||
    PREBUILT_METADATA_URL.endsWith('.gz')
  ) {
    return [PREBUILT_METADATA_URL]
  }
  return [
    `${PREBUILT_METADATA_URL}.zst`,
    `${PREBUILT_METADATA_URL}.gz`,
    PREBUILT_METADATA_URL,
  ]
}

async function decodeMetadataResponse(
  res: Response,
  url: string,
): Promise<PrebuiltMetadata> {
  if (url.endsWith('.gz')) {
    return await decodeCompressedMetadata(res, 'gzip')
  }

  const data = await res.json()
  return data as PrebuiltMetadata
}

async function decodeCompressedMetadata(
  res: Response,
  format: CompressionFormat,
): Promise<PrebuiltMetadata> {
  const bytes = new Uint8Array(await res.arrayBuffer())

  // Some hosts may transparently decompress based on content encoding.
  const plainText = new TextDecoder().decode(bytes)
  const plain = tryParseMetadata(plainText)
  if (plain) return plain

  if (typeof DecompressionStream === 'undefined') {
    throw new Error('Browser does not support DecompressionStream')
  }

  const decompressed = new Blob([bytes])
    .stream()
    .pipeThrough(new DecompressionStream(format))
  const text = await new Response(decompressed).text()
  const data = tryParseMetadata(text)
  if (!data) {
    throw new Error('Compressed metadata payload is not valid JSON')
  }
  return data
}

function tryParseMetadata(text: string): PrebuiltMetadata | null {
  try {
    return JSON.parse(text) as PrebuiltMetadata
  } catch {
    return null
  }
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
