import { CAP, MID, WIDTH, XH } from '../const'
import { corner, endOpen, open, simpleGlyph, spiro } from '../outline'
import { ringAt } from '../shape'

const strokeW = 0.17 * WIDTH
const halfStrokeW = 0.5 * strokeW

const strokeH = XH
const dotR = 0.65 * strokeW

export const i = simpleGlyph(
  spiro(
    open(MID, 0),
    corner(MID, strokeH - halfStrokeW),
    endOpen(MID - 0.4 * strokeH, strokeH - halfStrokeW),
  ).stroked(strokeW),
  ringAt(MID, CAP - dotR, dotR),
)
