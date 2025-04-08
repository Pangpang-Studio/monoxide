import { bezier, spiro, settings, glyph } from 'monoxide'

let g = glyph.simple((b) => {
  b.add(
    bezier(0.3, 0)
      .lineTo(0.3, 0)
      .lineTo(1, settings.width)
      .lineTo(0.3, 0)
      .build()
  )
})
glyph.assignChar(g, 'c')
