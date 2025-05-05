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
    let curves = part.kind.curve
    for (let i = 0; i < curves.length; i++) {
      debugSpiro(curves[i], points)
    }
  } else if (part.kind.t == 'cubic-bezier') {
    for (let i = 0; i < part.kind.curve.length; i++) {
      debugCubicBezier(part.kind.curve[i], points)
    }
  }

  if (part.result_curve && part.kind.t != 'spiro' && part.kind.t != 'stroke') {
    for (let i = 0; i < part.result_curve.length; i++) {
      let curve = part.result_curve[i]

      debugCubicBezier(curve, points)
    }

    if (part.debug_points) {
      for (let i = 0; i < part.debug_points.length; i++) {
        let point = part.debug_points[i]
        points.push({
          x: point.x,
          y: point.y,
          kind: point.kind,
          tag: point.tag,
        })
      }
    }
  }
  return points
}

function debugSpiro(curve: SerializeSpiroPoint[], points: SvgDebugPointInfo[]) {
  for (let j = 0; j < curve.length; j++) {
    let point = curve[j]
    points.push({
      x: point.x,
      y: point.y,
      kind: point.ty,
    })
  }
}

function debugCubicBezier(curve: CubicBezier, points: SvgDebugPointInfo[]) {
  console.log(curve)
  {
    let startPointKind: DebugPointKind = 'corner'
    if (curve.segments[0].t === 'curve') {
      startPointKind = 'curve'
    } else if (curve.segments[curve.segments.length - 1].t === 'curve') {
      startPointKind = 'curve'
    }
    points.push({
      x: curve.start.x,
      y: curve.start.y,
      kind: startPointKind,
    })
  }

  // Segments
  for (let j = 0; j < curve.segments.length; j++) {
    let segment = curve.segments[j]
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
      let nextSegment = curve.segments[j + 1]
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
