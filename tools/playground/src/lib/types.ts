// API is served at /api, the server will handle the reverse proxies

export interface NewRenderedMsg {
  t: 'NewRendered'
  glyphs: GlyphOverview[]
  cmap: Cmap
}

export interface ErrorMsg {
  t: 'Error'
  msg: string
}

export type WSRecvMsg = NewRenderedMsg | ErrorMsg

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

/** Maps to `struct FontOverview` in `model.rs` */
export interface FontOverview {
  glyphs: GlyphOverviewDetail[]
}

/** Maps to `struct GlyphOverviewDetail` in `model.rs` */
export interface GlyphOverviewDetail {
  id: number
  ch: string | null
  name: string | null
  outline: CubicBezier[]
}

export interface Guideline {
  pos: number
  label: string | null
}

export interface Guidelines {
  h: Guideline[]
  v: Guideline[]
}

/** Maps to `struct GlyphDetail` in `model.rs` */
export interface GlyphDetail {
  overview: GlyphOverviewDetail
  guidelines: Guidelines
  construction: SerializedGlyphConstruction[]
  result_id: number | null
  errors: string[]
}

/** Maps to `struct SerializedGlyphConstruction` in `model.rs` */
export interface SerializedGlyphConstruction {
  id: number
  kind: ConstructionKind
  result_curve: CubicBezier[] | null
  debug_points: DebugPoint[]
  debug_lines: DebugLine[]
}

/** Maps to `struct Point2D` in `cube.rs` */
export interface Point2D {
  x: number
  y: number
}

/** Maps to `CubicSegment::Line` variant in `cube.rs` */
export interface LineSegment {
  t: 'line'
  p2: Point2D
}

/** Maps to `CubicSegment::Curve` variant in `cube.rs` */
export interface CurveSegment {
  t: 'curve'
  c1: Point2D
  c2: Point2D
  p2: Point2D
}

/** Maps to `enum CubicSegment` in `cube.rs` */
export type CubicSegment = LineSegment | CurveSegment

/** Maps to `struct CubicBezier` in `cube.rs` */
export interface CubicBezier {
  start: Point2D
  segments: CubicSegment[]
  closed: boolean
}

export type DebugPointKind = 'corner' | 'curve' | 'control' | 'misc' | 'hidden'

/** Maps to `struct DebugPoint` in `model.rs` */
export interface DebugPoint extends Point2D {
  kind: DebugPointKind
  tag: string
}

/** Maps to `struct DebugLine` in `model.rs` */
export interface DebugLine {
  from: Point2D
  to: Point2D
  tag: string
}

/** Maps to `enum SerializeSpiroKind` in `model.rs` */
export type SerializeSpiroKind =
  | 'corner'
  | 'g4'
  | 'g2'
  | 'flat'
  | 'curl'
  | 'anchor'
  | 'handle'
  | 'open'
  | 'end-open'

/** Maps to `struct SerializeSpiroPoint` in `model.rs` */
export interface SerializeSpiroPoint extends Point2D {
  ty: SerializeSpiroKind
}

/** Maps to `ConstructionKind::Spiro` variant in `model.rs` */
export interface SpiroConstruction {
  t: 'spiro'
  curve: SerializeSpiroPoint[][]
}

/** Maps to `ConstructionKind::CubicBezier` variant in `model.rs` */
export interface CubicBezierConstruction {
  t: 'cubic-bezier'
  curve: CubicBezier[]
}

/** Maps to `ConstructionKind::Stroke` variant in `model.rs` */
export interface StrokeConstruction {
  t: 'stroke'
  parent: number
  width: number
  curve: SerializeSpiroPoint[][]
}

/** Maps to `ConstructionKind::SpiroToBezier` variant in `model.rs` */
export interface SpiroToBezierConstruction {
  t: 'spiro-to-bezier'
  parent: number
}

/** Maps to `ConstructionKind::BooleanAdd` variant in `model.rs` */
export interface BooleanAddConstruction {
  t: 'boolean-add'
  parents: number[]
}

/** Maps to `enum ConstructionKind` in `model.rs` */
export type ConstructionKind =
  | SpiroConstruction
  | CubicBezierConstruction
  | StrokeConstruction
  | SpiroToBezierConstruction
  | BooleanAddConstruction
