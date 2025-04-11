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
    const builder = builderFactory()
    for (const { meth, args } of insts) builder[meth](...args)
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
 * @param {SpiroInst[]} insts
 * @returns {import('monoxide').OutlineExpr}
 */
export const spiro = instCollector(mx.spiro)

export const corner = instFactory('corner')
export const g4 = instFactory('g4')
export const g2 = instFactory('g2')
export const flat = instFactory('flat')
export const curl = instFactory('curl')
export const anchor = instFactory('anchor')
export const handle = instFactory('handle')
export const end = instFactory('end')
export const open = instFactory('open')
export const endOpen = instFactory('endOpen')

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

export const moveTo = instFactory('moveTo')
export const lineTo = instFactory('lineTo')
export const curveTo = instFactory('curveTo')
export const close = instFactory('close')
