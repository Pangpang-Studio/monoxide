interface GlyphFactory {
  readonly brand: unique symbol
  simple(b: SimpleGlyphBuilder): GlyphEntry
}

interface SimpleGlyphBuilder {
  readonly brand: unique symbol
  add(o: OutlineExpr): SimpleGlyphBuilder
}

interface BezierBuilder {
  readonly brand: unique symbol
  lineTo(x: number, y: number): BezierBuilder
  curveTo(x1: number, y1: number, x2: number, y2: number, x3: number, y3: number): BezierBuilder
  close(): BezierBuilder
  build(): OutlineExpr
}

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
  build(): OutlineExpr
}

interface GlyphEntry {
  readonly brand: unique symbol

  char(ch: string): GlyphEntry
}

/// Represents an outline built
interface OutlineExpr {
  readonly brand: unique symbol

  stroked(width: number): OutlineExpr
}

declare const glyph: GlyphFactory
declare function bezier(x: number, y: number): BezierBuilder
declare function spiro(): SpiroBuilder
