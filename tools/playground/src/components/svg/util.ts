import type { Point2D } from '../../lib/types'
import type { DraftToSvgXform } from './types'

export function xform(cvt: DraftToSvgXform, x: number, y: number): Point2D {
  return {
    x: cvt.scaleX * x + cvt.translateX,
    y: cvt.scaleY * y + cvt.translateY,
  }
}

export function xformPoint(cvt: DraftToSvgXform, p: Point2D): Point2D {
  return xform(cvt, p.x, p.y)
}

export function xformX(cvt: DraftToSvgXform, x: number): number {
  return cvt.scaleX * x + cvt.translateX
}

export function xformY(cvt: DraftToSvgXform, y: number): number {
  return cvt.scaleY * y + cvt.translateY
}
