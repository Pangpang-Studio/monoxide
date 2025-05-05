import type { CubicBezier, Point2D } from '../../lib/types'
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

/**
 * Convert a cubic bezier curve to SVG path data.
 *
 * @param cvt The conversion object for transforming coordinates
 * @param curve The cubic bezier curve to be converted to SVG path data
 */
export function svgPen(cvt: DraftToSvgXform, curve: CubicBezier): string {
  const start = xformPoint(cvt, curve.start)
  let path = `M${start.x} ${start.y}`
  for (let i = 0; i < curve.segments.length; i++) {
    const segment = curve.segments[i]
    if (segment.t === 'curve') {
      const c1 = xformPoint(cvt, segment.c1)
      const c2 = xformPoint(cvt, segment.c2)
      const p2 = xformPoint(cvt, segment.p2)
      path += ` C${c1.x} ${c1.y}, ${c2.x} ${c2.y}, ${p2.x} ${p2.y}`
    } else if (segment.t === 'line') {
      const p2 = xformPoint(cvt, segment.p2)
      path += ` L${p2.x} ${p2.y}`
    }
  }
  return path
}

/**
 * Convert multiple cubic bezier curves to SVG path data.
 *
 * @param cvt The conversion object for transforming coordinates
 * @param curves The array of cubic bezier curves to be converted to SVG path
 *   data
 */
export function svgPenMulti(
  cvt: DraftToSvgXform,
  curves: CubicBezier[],
): string {
  return curves.map((curve) => svgPen(cvt, curve)).join(' ')
}
