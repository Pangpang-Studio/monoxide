import mx from 'monoxide'

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
 * @returns {(insts: Inst[]) => import('monoxide').OutlineExpr}
 */
function instCollector(builderFactory) {
  return (insts) => {
    const builder = builderFactory()
    for (const { meth, args } of insts) builder[meth](...args)
    return builder.build()
  }
}

/**
 * @param {SpiroInst[]} insts
 * @returns {import('monoxide').OutlineExpr}
 */
const spiro = instCollector(mx.spiro)

/**
 * @typedef {'lineTo' | 'curveTo' | 'close'} BezierMeth
 *
 * @typedef {Inst & { meth: BezierMeth }} BezierInst
 */

/**
 * @param {BezierInst[]} insts
 * @returns {import('monoxide').OutlineExpr}
 */
const bezier = instCollector(() => mx.bezier)
