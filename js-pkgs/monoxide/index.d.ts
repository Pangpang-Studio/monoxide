interface GlyphFactory {
  readonly brand: unique symbol
  simple(f: (b: SimpleGlyphBuilder) => void): GlyphEntry

  assignChar(glyph: GlyphEntry, char: string): void
  // TODO: more complex settings, variants, etc.
}

interface SimpleGlyphBuilder {
  readonly brand: unique symbol
  add(o: OutlineExpr): SimpleGlyphBuilder
}

/**
 * A builder for bezier curves. Single-use.
 */
interface BezierBuilder {
  readonly brand: unique symbol
  lineTo(x: number, y: number): BezierBuilder
  curveTo(
    x1: number,
    y1: number,
    x2: number,
    y2: number,
    x3: number,
    y3: number
  ): BezierBuilder
  close(): BezierBuilder

  /**
   * Get the bezier curve and invalidates this builder.
   */
  build(): OutlineExpr
}

/**
 * A builder for spiro curves. Single-use.
 */
interface SpiroBuilder {
  readonly brand: unique symbol
  g4(x: number, y: number): SpiroBuilder
  g2(x: number, y: number): SpiroBuilder
  flat(x: number, y: number): SpiroBuilder
  curl(x: number, y: number): SpiroBuilder
  anchor(x: number, y: number): SpiroBuilder
  handle(x: number, y: number): SpiroBuilder
  end(x: number, y: number): SpiroBuilder
  open(x: number, y: number): SpiroBuilder
  endOpen(x: number, y: number): SpiroBuilder

  /**
   * Get the spiro curve and invalidates this builder.
   */
  build(): OutlineExpr
}

/**
 * Represents a built outline in Rust side
 */
interface OutlineExpr {
  readonly brand: unique symbol

  stroked(width: number): OutlineExpr
}

/**
 * Opaque type representing a glyph
 */
interface GlyphEntry {
  readonly brand: unique symbol
}

/**
 * Global settings of the font. For lengths, font size = 1.
 */
interface Settings {
  width: number
  xHeight: number
  descender: number
  capHeight: number
}

declare const settings: Settings
declare const glyph: GlyphFactory
declare function bezier(x: number, y: number): BezierBuilder
declare function spiro(): SpiroBuilder
