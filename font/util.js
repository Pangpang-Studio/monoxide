import * as mx from 'monoxide'

/** @typedef {{ meth: string; args: number[] }} Inst */

/**
 * @typedef {'corner'
 *   | 'g4'
 *   | 'g2'
 *   | 'flat'
 *   | 'curl'
 *   | 'anchor'
 *   | 'handle'
 *   | 'end'
 *   | 'open'
 *   | 'endOpen'} SpiroMeth
 *
 *
 * @typedef {Inst & { meth: SpiroMeth }} SpiroInst
 */

/**
 * @param {() => any} builderFactory
 * @returns {(...insts: Inst[]) => import('monoxide').OutlineExpr}
 */
function instCollector(builderFactory) {
  return (...insts) => {
    let builder = builderFactory()
    for (const { meth, args } of insts) builder = builder[meth](...args)
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
 * @typedef {(...args: [number, number]) => Inst} PointInstFactory
 * @param {string} meth
 * @returns {PointInstFactory}
 */
const pointInstFactory = (meth) => instFactory(meth)

/**
 * @param {SpiroInst[]} insts
 * @returns {import('monoxide').OutlineExpr}
 */
export const spiro = instCollector(mx.spiro)

export const corner = pointInstFactory('corner')
export const g4 = pointInstFactory('g4')
export const g2 = pointInstFactory('g2')
export const flat = pointInstFactory('flat')
export const curl = pointInstFactory('curl')
export const anchor = pointInstFactory('anchor')
export const handle = pointInstFactory('handle')
export const end = pointInstFactory('end')
export const open = pointInstFactory('open')
export const endOpen = pointInstFactory('endOpen')

/**
 * @typedef {'moveTo' | 'lineTo' | 'curveTo' | 'close'} BezierMeth
 *
 * @typedef {Inst & { meth: BezierMeth }} BezierInst
 */

/**
 * @param {BezierInst[]} insts
 * @returns {import('monoxide').OutlineExpr}
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
