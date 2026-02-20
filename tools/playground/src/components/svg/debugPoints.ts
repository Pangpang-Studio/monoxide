import type {
  CubicBezier,
  DebugPointKind,
  SerializedGlyphConstruction,
  SerializeSpiroPoint,
} from '../../lib/types'
import type { SvgDebugPointInfo } from './types'

export function generateDebugPoints(
  part: SerializedGlyphConstruction,
): SvgDebugPointInfo[] {
  let points: SvgDebugPointInfo[] = []

  if (part.kind.t == 'spiro' || part.kind.t == 'stroke') {
    for (const curve of part.kind.curve) debugSpiro(curve, points)
  } else {
    if (part.kind.t == 'cubic-bezier' || part.kind.t == 'transform') {
      for (const curve of part.kind.curve) debugCubicBezier(curve, points)
    }

    if (part.result_curve) {
      for (const curve of part.result_curve) debugCubicBezier(curve, points)
      if (part.debug_points) {
        for (const point of part.debug_points) {
          points.push({
            x: point.x,
            y: point.y,
            kind: point.kind,
            tag: point.tag,
          })
        }
      }
    }
  }
  return points
}

function debugSpiro(curve: SerializeSpiroPoint[], points: SvgDebugPointInfo[]) {
  for (const point of curve) {
    points.push({
      x: point.x,
      y: point.y,
      kind: point.ty,
    })
  }
}

function debugCubicBezier(curve: CubicBezier, points: SvgDebugPointInfo[]) {
  console.log(curve)
  if (curve.segments.length === 0) return

  {
    let startPointKind: DebugPointKind = 'corner'
    if (
      curve.segments[0]!.t === 'curve' ||
      curve.segments[curve.segments.length - 1]!.t === 'curve'
    )
      startPointKind = 'curve'

    points.push({
      x: curve.start.x,
      y: curve.start.y,
      kind: startPointKind,
    })
  }

  // Segments
  for (let j = 0; j < curve.segments.length; j++) {
    let segment = curve.segments[j]!
    if (segment.t === 'curve') {
      points.push({
        x: segment.c1.x,
        y: segment.c1.y,
        kind: 'control',
      })
      points.push({
        x: segment.c2.x,
        y: segment.c2.y,
        kind: 'control',
      })
    } else if (segment.t === 'line') {
      // pass
    }
    if (j === curve.segments.length - 1) {
      if (curve.closed) {
        // pass, we already added the start point
      } else {
        points.push({
          x: segment.p2.x,
          y: segment.p2.y,
          kind: segment.t == 'curve' ? 'curve' : 'corner',
        })
      }
    } else {
      let nextSegment = curve.segments[j + 1]!
      let isCurve = segment.t === 'curve' || nextSegment.t === 'curve'
      if (isCurve) {
        points.push({
          x: segment.p2.x,
          y: segment.p2.y,
          kind: 'curve',
        })
      } else {
        points.push({
          x: segment.p2.x,
          y: segment.p2.y,
          kind: 'corner',
        })
      }
    }
  }
}
