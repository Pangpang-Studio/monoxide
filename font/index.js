import { glyph } from 'monoxide'
import { S, s } from './glyph/s'
import { c } from './glyph/c'
import { o } from './glyph/o'

const glyphs = {
  S,
  c,
  o,
  s,
}

for (const [ch, gl] of Object.entries(glyphs)) {
  glyph.assignChar(ch, gl)
}
