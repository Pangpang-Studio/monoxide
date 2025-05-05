import { settings } from 'monoxide'

/**
 * The standard directions.
 *
 * @type {Record<string, import('./point').Point>}
 */
export const DIR = {
  L: [-1, 0],
  R: [1, 0],
  U: [0, 1],
  D: [0, -1],
}

// === Original constants ===

/** The full width of a half-width character. */
export const WIDTH = settings.width

/** The x-height. */
export const XH = settings.xHeight

/** The cap height. */
export const CAP = settings.capHeight

/** The descender. */
export const DESC = settings.descender

/** The overshoot. */
export const OVRS = settings.overshoot

/** The left side bearing. */
export const SBL = settings.sideBearing

// === Derived constants ===

/** The horizontal midline of a half-width character. */
export const MID = 0.5 * WIDTH

/** The right side bearing. */
export const SBR = WIDTH - settings.sideBearing
