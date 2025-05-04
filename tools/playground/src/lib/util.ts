export interface CharListOutput {
  charcode: number
  unicode: string
  ch: string
}

export function charList(chars: string[]): CharListOutput[] {
  return chars.map((c) => {
    let n = c.codePointAt(0)
    if (n === undefined) {
      throw new Error(`Invalid character: ${c}`)
    }
    return {
      charcode: n,
      unicode: `u+${n.toString(16).padStart(4, '0')}`,
      ch: c,
    }
  })
}
