import { CAP, MID, SBL, SBR, WIDTH, XH } from '../const'
import { mix } from '../math'
import { corner, endOpen, open, simpleGlyph, spiro } from '../outline'
import { rect, ringAt } from '../shape'

const strokeW = 0.17 * WIDTH
const halfStrokeW = 0.5 * strokeW

const strokeH = XH
const dotR = 0.65 * strokeW

export const i = simpleGlyph(
  spiro(
    open(MID, strokeW),
    corner(MID, strokeH - halfStrokeW),
    endOpen(mix(MID, SBL, 1 / 5), strokeH - halfStrokeW),
  ).stroked(strokeW),
  rect(SBL, halfStrokeW, SBR, halfStrokeW, strokeW),
  ringAt(MID, CAP - dotR, dotR),
)
