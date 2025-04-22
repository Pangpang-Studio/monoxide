import { settings } from 'monoxide'
import { endOpen, g4, open, simpleGlyph, spiro } from '../outline'

/**
 * Renders a rectangle that is formed by drawing a line between points (x0, y0)
 * and (x1, y1) of the given width.
 *
 * @param {number} x0
 * @param {number} y0
 * @param {number} x1
 * @param {number} y1
 * @param {number} width
 * @returns {import('monoxide').OutlineExpr} The outline of the rectangle
 */
function rect(x0, y0, x1, y1, width) {
  return spiro(open(x0, y0), endOpen(x1, y1)).stroked(width)
}

/**
 * Returns the weighted average of two values.
 *
 * @param {number} z
 * @param {number} w
 * @param {number} ratio The weighting factor of z
 * @returns {number} The weighted average of z and w
 */
function mix(z, w, ratio) {
  return w + ratio * (z - w)
}

/**
 * Renders a ring that is delimited by drawing a line between the coordinates
 * (x0, x1) and (y0, y1).
 *
 * @param {number} x0
 * @param {number} y0
 * @param {number} x1
 * @param {number} y1
 * @returns {import('monoxide').OutlineExpr} The outline of the ring
 */
function ring(x0, y0, x1, y1) {
  const xh = mix(x0, x1, 0.5)
  const yh = mix(y0, y1, 0.5)
  return spiro(g4(x0, yh), g4(xh, y0), g4(x1, yh), g4(xh, y1))
}

const { width, descender, xHeight } = settings
const mid = 0.5 * width
const strokeWidth = 0.17 * width

/** @typedef {[number, number]} Point */

/** @type {Point} */
const iStrokeBot = [mid, descender]

/** @type {Point} */
const iStrokeTop = [mid, xHeight]

const iStrokeHeight = iStrokeTop[1] - iStrokeBot[1]

export const i = simpleGlyph(
  rect(...iStrokeBot, ...iStrokeTop, strokeWidth),
  rect(
    iStrokeTop[0] - 0.5 * strokeWidth,
    iStrokeTop[1] - 0.5 * strokeWidth,
    iStrokeTop[0] - 0.35 * iStrokeHeight,
    iStrokeTop[1] - 0.5 * strokeWidth,
    strokeWidth,
  ),
  ring(
    iStrokeTop[0] - 0.75 * strokeWidth,
    iStrokeTop[1] + 1 * strokeWidth,
    iStrokeTop[0] + 0.75 * strokeWidth,
    iStrokeTop[1] + 2.5 * strokeWidth,
  ),
)
