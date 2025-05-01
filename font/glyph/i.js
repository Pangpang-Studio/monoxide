import { settings } from 'monoxide'

import { corner, endOpen, open, simpleGlyph, spiro } from '../outline'
import { ringAt } from '../shape'

const { width, xHeight, capHeight } = settings
const mid = 0.5 * width
const strokeW = 0.17 * width
const halfStrokeW = 0.5 * strokeW

const strokeH = xHeight
const dotR = 0.65 * strokeW

export const i = simpleGlyph(
  spiro(
    open(mid, 0),
    corner(mid, strokeH - halfStrokeW),
    endOpen(mid - 0.4 * strokeH, xHeight - halfStrokeW),
  ).stroked(strokeW),
  ringAt(mid, capHeight - dotR, dotR),
)
