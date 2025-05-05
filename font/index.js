import { glyph } from 'monoxide'

import { c } from './glyph/c'
import { i } from './glyph/i'
import { n } from './glyph/n'
import { o } from './glyph/o'
import { S, s } from './glyph/s'

const glyphs = {
  S,
  c,
  i,
  n,
  o,
  s,
}

for (const [ch, gl] of Object.entries(glyphs)) {
  glyph.assignChar(ch, gl)
}
