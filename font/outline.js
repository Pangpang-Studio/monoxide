import * as mx from 'monoxide'

/** @import {Point} from "./point" */

/**
 * @typedef {object} Inst
 * @property {string} meth
 * @property {number[]} args
 * @property {Record<string, number[]>} [extras]
 */

/**
 * @typedef {'corner'
 *   | 'g4'
 *   | 'g2'
 *   | 'flat'
 *   | 'curl'
 *   | 'anchor'
 *   | 'handle'
 *   | 'open'
 *   | 'endOpen'} SpiroMeth
 */

/**
 * @augments {Inst}
 * @class
 */
class SpiroInst {
  /** @type {SpiroMeth} */
  meth
  /** @type {number[]} */
  args
  /** @type {Record<string, number[]>} */
  extras = {}

  /**
   * @param {SpiroMeth} meth
   * @param {...number} args
   */
  constructor(meth, ...args) {
    this.meth = meth
    this.args = args
  }

  /**
   * @param {number} x
   * @param {number} y
   * @returns {SpiroInst}
   */
  heading(x, y) {
    this.extras.heading = [x, y]
    return this
  }
}

/**
 * @param {() => any} builderFactory
 * @returns {(...insts: Inst[]) => mx.OutlineExpr}
 */
function instCollector(builderFactory) {
  return (...insts) => {
    let builder = builderFactory()
    for (const { meth, args, extras } of insts) {
      builder = builder[meth](...args)
      if (extras)
        for (const [meth, args] of Object.entries(extras))
          builder = builder[meth](...args)
    }
    return builder.build()
  }
}

/**
 * @typedef {(...args: number[]) => Inst} InstFactory
 * @param {string} meth
 * @returns {InstFactory}
 */
const instFactory =
  (meth) =>
  (...args) => ({ meth, args })

/**
 * @typedef {(...args: Point) => Inst} PointInstFactory
 * @param {string} meth
 * @returns {PointInstFactory}
 */
const pointInstFactory = (meth) => instFactory(meth)

/**
 * @typedef {(...args: Point) => SpiroInst} SpiroInstFactory
 * @param {SpiroMeth} meth
 * @returns {SpiroInstFactory}
 */
const spiroInstFactory =
  (meth) =>
  (...args) =>
    new SpiroInst(meth, ...args)

/**
 * @param {SpiroInst[]} insts
 * @returns {mx.OutlineExpr}
 */
export const spiro = instCollector(mx.spiro)

export const corner = spiroInstFactory('corner')
export const g4 = spiroInstFactory('g4')
export const g2 = spiroInstFactory('g2')
export const flat = spiroInstFactory('flat')
export const curl = spiroInstFactory('curl')
export const anchor = spiroInstFactory('anchor')
export const handle = spiroInstFactory('handle')
export const open = spiroInstFactory('open')
export const endOpen = spiroInstFactory('endOpen')

/**
 * @typedef {'moveTo' | 'lineTo' | 'curveTo' | 'close'} BezierMeth
 *
 * @typedef {Inst & { meth: BezierMeth }} BezierInst
 */

/**
 * @param {BezierInst[]} insts
 * @returns {mx.OutlineExpr}
 */
export const bezier = instCollector(() => ({ moveTo: mx.bezier }))

export const moveTo = pointInstFactory('moveTo')
export const lineTo = pointInstFactory('lineTo')

/**
 * @type {(
 *   ...args: [number, number, number, number, number, number]
 * ) => Inst}
 */
export const curveTo = instFactory('curveTo')

/** @type {() => Inst} */
export const close = instFactory('close')

/**
 * @param {mx.OutlineExpr[]} outlines
 * @returns {mx.GlyphEntry}
 */
export function simpleGlyph(...outlines) {
  return mx.glyph.simple((builder) => {
    for (const outline of outlines) builder.add(outline)
  })
}
