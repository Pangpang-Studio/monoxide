import { settings, glyph as gl } from 'monoxide'
import {
  bezier,
  corner,
  endOpen,
  g2,
  g4,
  lineTo,
  moveTo,
  open,
  simpleGlyph,
  spiro,
} from './util.js'

gl.assignChar(
  'c',
  simpleGlyph(
    bezier(
      moveTo(0.3, 0),
      lineTo(0.6, 0),
      lineTo(1, settings.width),
      lineTo(0.3, 0),
    ),
  ),
)

gl.assignChar(
  's',
  simpleGlyph(
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
  ),
)

gl.assignChar(
  'S',
  simpleGlyph(
    spiro(
      open(0.305, 0.176),
      g2(0.212, 0.142),
      g2(0.159, 0.171),
      g2(0.224, 0.237),
      g2(0.347, 0.335),
      g2(0.202, 0.467),
      endOpen(0.081, 0.429),
    ).stroked(0.05),
  ),
)

gl.assignChar(
  'o',
  simpleGlyph(spiro(g4(0, 0.5), g4(0.5, 0), g4(1, 0.5), g4(0.5, 1))),
)
