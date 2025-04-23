import { settings } from 'monoxide'

import { simpleGlyph } from '../outline'
import { rect, ring } from '../shape'

/** @import {Point} from '../shape' */

const { width, descender, xHeight } = settings
const mid = 0.5 * width
const strokeWidth = 0.17 * width

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
