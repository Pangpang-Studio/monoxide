import type { GlyphDetail } from './types'

export async function ping() {
  let res = await fetch('/api/ping')
  if (!res.ok) {
    throw new Error('Failed to ping API')
  }
}

export async function getGlyphDetail(id: number) {
  let res = await fetch(`/api/glyph/${id}`)
  if (!res.ok) {
    let body = await res.text()
    throw new Error(`Failed to fetch glyph detail: ${res.statusText}; ${body}`)
  }
  let data = await res.json()
  return data as GlyphDetail
}
