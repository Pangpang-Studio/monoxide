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

export function drawSvgDirection(bez: CubicBezier, cvt: DraftToSvgXform) {
  if (bez.segments.length === 0) {
    return null
  }
  let seg0 = bez.segments[0]
  let p2: Point2D
  if (seg0.t == 'line') {
    p2 = seg0.p2
  } else {
    p2 = seg0.c1
  }
  let p1 = bez.start

  // Direction from p1 to p2
  let dx = p2.x - p1.x
  let dy = p2.y - p1.y
  let d = Math.sqrt(dx * dx + dy * dy)
  if (d < 1e-8) {
    return null
  }

  // Marker line
  let line_len = 0.04
  const tip = {
    x: p1.x + (dx / d) * line_len,
    y: p1.y + (dy / d) * line_len,
  }

  // Arrowhead length (tip to base distance)
  const L = 0.01

  // unit direction vector
  const ux = dx / d
  const uy = dy / d

  // base center is L behind the tip along the direction
  const bx = tip.x - L * ux
  const by = tip.y - L * uy

  // perpendicular unit vector
  const vx = -uy
  const vy = ux

  // for a 90deg tip angle (half-angle 45deg), half-width = L
  const halfWidth = L

  const b1 = { x: bx + halfWidth * vx, y: by + halfWidth * vy }
  const b2 = { x: bx - halfWidth * vx, y: by - halfWidth * vy }

  // Path: tip -> b1 -> b2 -> close
  let p1x = xformPoint(cvt, p1)
  let tipx = xformPoint(cvt, tip)
  let b1x = xformPoint(cvt, b1)
  let b2x = xformPoint(cvt, b2)

  return `M ${p1x.x} ${p1x.y} L ${tipx.x} ${tipx.y} L ${b1x.x} ${b1x.y} L ${b2x.x} ${b2x.y} L ${tipx.x} ${tipx.y} Z`
}
