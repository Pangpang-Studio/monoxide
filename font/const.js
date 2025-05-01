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

export const WIDTH = settings.width
export const XH = settings.xHeight
export const CAP = settings.capHeight
export const DESC = settings.descender
export const OVRS = settings.overshoot

export const MID = 0.5 * WIDTH
