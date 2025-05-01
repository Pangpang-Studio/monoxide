import { MID, OVRS, WIDTH, XH } from '../const'
import { curl, flat, g2, g4, simpleGlyph, spiro } from '../outline'

const strokeW = 0.17 * XH

const XW = 0.57 * WIDTH
const halfXW = 0.5 * XW
const midCurveW = 0.3 * XW

const halfXH = 0.5 * XH
const midCurveH = (1 / 17) * halfXH
const endCurveH = (9 / 17) * halfXH

const endN = g2
const midN = g4

export const o = simpleGlyph(
  spiro(
    midN(MID - midCurveW, midCurveH),
    endN(MID, -OVRS),
    midN(MID + midCurveW, midCurveH),
    flat(MID + halfXW, endCurveH),
    curl(MID + halfXW, XH - endCurveH),
    midN(MID + midCurveW, XH - midCurveH),
    endN(MID, XH + OVRS),
    midN(MID - midCurveW, XH - midCurveH),
    flat(MID - halfXW, XH - endCurveH),
    curl(MID - halfXW, endCurveH),
  ), // .stroked(strokeW),
)
