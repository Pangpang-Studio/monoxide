import { glyph } from 'monoxide'
import { S, s } from './glyph/s'
import { c } from './glyph/c'
import { i } from './glyph/i'
import { o } from './glyph/o'

const glyphs = {
  S,
  c,
  i,
  o,
  s,
}

for (const [ch, gl] of Object.entries(glyphs)) {
  glyph.assignChar(ch, gl)
}
