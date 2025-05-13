import { DIR, MID, OVRS, SBL, SBR, XH } from '../const'
import { curl, endOpen, g4, open, simpleGlyph, spiro } from '../outline'

// TODO: Extract this.
const strokeW = 0.17 * XH

const endN = g4

export const n = (() => {
  const sbl1 = SBL + strokeW / 2
  const sbr1 = SBR - strokeW / 2

  const midCurveW = 0.2 * (SBR - SBL)
  const ry = (XH - strokeW) / 2
  const endCurveH = (9 / 17) * ry
  const yHi = XH / 2 + ry

  return simpleGlyph(
    spiro(
      open(sbr1, 0),
      curl(sbr1, yHi - endCurveH),
      endN(MID + midCurveW, yHi + OVRS / 2),
      endOpen(sbl1 + strokeW / 2, yHi - strokeW / 2).heading(...DIR.L),
    ).stroked(strokeW),
    spiro(open(sbl1, 0), endOpen(sbl1, XH)).stroked(strokeW),
  )
})()
