import { corner, endOpen, g2, open, simpleGlyph, spiro } from '../outline'

export const s = simpleGlyph(
  spiro(
    corner(0.334, 0.117),
    corner(0.305, 0.176),
    g2(0.212, 0.142),
    g2(0.159, 0.171),
    g2(0.224, 0.237),
    g2(0.347, 0.335),
    g2(0.202, 0.467),
    corner(0.081, 0.429),
    corner(0.114, 0.368),
    g2(0.201, 0.402),
    g2(0.276, 0.369),
    g2(0.218, 0.308),
    g2(0.091, 0.211),
    g2(0.124, 0.111),
    g2(0.229, 0.082),
  ),
)

export const S = simpleGlyph(
  spiro(
    open(0.305, 0.176),
    g2(0.212, 0.142),
    g2(0.159, 0.171),
    g2(0.224, 0.237),
    g2(0.347, 0.335),
    g2(0.202, 0.467),
    endOpen(0.081, 0.429).heading(-1, 0),
  ).stroked(0.05),
)
