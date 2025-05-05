import type { CubicBezier } from '../../lib/types'

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
}

export enum SelectionMode {
  None = 'none',
  Part = 'part',
  Overlay = 'overlay',
}
