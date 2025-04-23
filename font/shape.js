import { mix } from './math'
import { endOpen, g4, open, spiro } from './outline'

/** @typedef {[number, number]} Point */

/**
 * Renders a rectangle formed by drawing a line between points (x0, y0) and (x1,
 * y1) of the given width.
 *
 * @param {number} x0
 * @param {number} y0
 * @param {number} x1
 * @param {number} y1
 * @param {number} width
 * @returns {import('monoxide').OutlineExpr} The outline of the rectangle
 */
export function rect(x0, y0, x1, y1, width) {
  return spiro(open(x0, y0), endOpen(x1, y1)).stroked(width)
}

/**
 * Renders a ring that is delimited within x and y ranges [x0, x1] and [y0, y1].
 *
 * @param {number} x0
 * @param {number} y0
 * @param {number} x1
 * @param {number} y1
 * @returns {import('monoxide').OutlineExpr} The outline of the ring
 */
export function ring(x0, y0, x1, y1) {
  const xh = mix(x0, x1, 0.5)
  const yh = mix(y0, y1, 0.5)
  return spiro(g4(x0, yh), g4(xh, y0), g4(x1, yh), g4(xh, y1))
}
