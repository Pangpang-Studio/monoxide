// API is served at /api, the server will handle the reverse proxies

export interface NewRenderedMsg {
  t: 'NewRendered'
  glyphs: GlyphOverview[]
  cmap: Cmap
}

export interface GlyphOverview {
  svg: string
  x0: number
  y0: number
  x1: number
  y1: number
}

export interface Cmap {
  [key: string]: number
}

export interface ErrorMsg {
  t: 'Error'
  msg: string
}

export type WSRecvMsg = NewRenderedMsg | ErrorMsg

export async function ping() {
  let res = await fetch('/api/ping')
  if (!res.ok) {
    throw new Error('Failed to ping API')
  }
}
