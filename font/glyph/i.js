import { settings } from 'monoxide'

import { corner, endOpen, open, simpleGlyph, spiro } from '../outline'
import { ringAt } from '../shape'

/** @import {Point} from '../point' */

const { width, descender, xHeight } = settings
const mid = 0.5 * width
const strokeW = 0.17 * width
const halfStrokeW = 0.5 * strokeW

/** @type {Point} */
const iStrokeBot = [mid, descender]

/** @type {Point} */
const iStrokeTop = [mid, xHeight]

const iStrokeHeight = iStrokeTop[1] - iStrokeBot[1]

export const i = simpleGlyph(
  spiro(
    open(mid, descender),
    corner(mid, xHeight - halfStrokeW),
    endOpen(mid - 0.4 * iStrokeHeight, xHeight - halfStrokeW),
  ).stroked(strokeW),
  ringAt(mid, 1.225 * xHeight, 0.65 * strokeW),
)
