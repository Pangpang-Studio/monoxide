import { g4, simpleGlyph, spiro } from '../outline'

export const o = simpleGlyph(
  spiro(g4(0, 0.5), g4(0.5, 0), g4(1, 0.5), g4(0.5, 1)),
)
