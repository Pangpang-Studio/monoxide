import { MID, OVRS, SBL, SBR, XH } from '../const'
import {
  corner,
  curl,
  endOpen,
  flat,
  g2,
  open,
  simpleGlyph,
  spiro,
} from '../outline'

// TODO: Extract this.
const strokeW = 0.17 * XH

const endN = g2

export const n = (() => {
  const sbl1 = SBL + strokeW / 2
  const sbr1 = SBR - strokeW / 2

  const midCurveW = 0.2 * (SBR - SBL)
  const ry = (XH - strokeW) / 2
  const midCurveH = (1 / 17) * ry
  const endCurveH = (9 / 17) * ry
  const yHi = XH / 2 + ry

  return simpleGlyph(
    spiro(
      open(sbr1, 0),
      curl(sbr1, yHi - endCurveH),
      endN(MID + midCurveW, yHi - midCurveH),
      flat(MID - midCurveW, yHi + OVRS),
      corner(sbl1, yHi + OVRS),
      endOpen(sbl1, 0),
    ).stroked(strokeW),
  )
})()
