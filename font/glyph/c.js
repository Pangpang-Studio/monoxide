import { settings } from 'monoxide'
import { bezier, lineTo, moveTo, simpleGlyph } from '../util'

export const c = simpleGlyph(
  bezier(
    moveTo(0.3, 0),
    lineTo(0.6, 0),
    lineTo(1, settings.width),
    lineTo(0.3, 0),
  ),
)
