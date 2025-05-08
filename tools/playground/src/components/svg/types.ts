import type {
  CubicBezier,
  DebugPointKind,
  SerializeSpiroKind,
} from '../../lib/types'

export interface SvgGuideline {
  pos: number
  label: string | null
}

export interface SvgGuidelines {
  h: SvgGuideline[]
  v: SvgGuideline[]
}

export interface DraftToSvgXform {
  scaleX: number
  scaleY: number
  translateX: number
  translateY: number
}

export type AcceptedKinds = DebugPointKind | SerializeSpiroKind

export interface SvgDebugPointInfo {
  x: number
  y: number
  kind: AcceptedKinds
  /** The forward angle, in degrees. Only used in {forward,backward}-triangle */
  forward?: number
  tag?: string
}

export interface SvgGuidelinesProps extends SvgGuidelines {
  xform: DraftToSvgXform
  canvasWidth: number
  canvasHeight: number
}

export interface SvgCanvasProps {
  guidelines: { h: SvgGuideline[]; v: SvgGuideline[] }
  /**
   * 1-based margin on each sides. If not supplied, defaults to 0.2. Margins are
   * relative to the extremeties of the guidelines.
   */
  margin?: number
  mainPath: CubicBezier[] | null
  selected: SelectionMode
  debugPaths: CubicBezier[]
  debugFill: CubicBezier[]
  debugPoints: SvgDebugPointInfo[]
}

export enum SelectionMode {
  None = 'none',
  Part = 'part',
  Overlay = 'overlay',
}
