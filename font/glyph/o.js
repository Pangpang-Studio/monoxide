import { MID, OVRS, SBL, XH } from '../const'
import { curl, flat, g2, g4, simpleGlyph, spiro } from '../outline'

const strokeW = 0.17 * XH

const endN = g2
const midN = g4

/**
 * @param {number} x
 * @param {number} y
 * @param {number} rx
 * @param {number} ry
 * @returns {import('monoxide').OutlineExpr}
 */
function oShape(x, y, rx, ry) {
  const midCurveW = 0.6 * rx
  const midCurveH = (1 / 17) * ry
  const endCurveH = (9 / 17) * ry
  const yLo = y - ry
  const yHi = y + ry

  return spiro(
    midN(x - midCurveW, yLo + midCurveH),
    endN(x, yLo - OVRS),
    midN(x + midCurveW, yLo + midCurveH),
    flat(x + rx, yLo + endCurveH),
    curl(x + rx, yHi - endCurveH),
    midN(x + midCurveW, yHi - midCurveH),
    endN(x, yHi + OVRS),
    midN(x - midCurveW, yHi - midCurveH),
    flat(x - rx, yHi - endCurveH),
    curl(x - rx, yLo + endCurveH),
  ).stroked(strokeW)
}

export const o = simpleGlyph(
  oShape(MID, XH / 2, MID - SBL - strokeW / 2, (XH - strokeW) / 2),
)
