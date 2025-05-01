/** @typedef {[number, number]} Point */

/**
 * The standard directions.
 *
 * @type {Record<string, Point>}
 */
export const DIR = {
  L: [-1, 0],
  R: [1, 0],
  U: [0, 1],
  D: [0, -1],
}

/**
 * @param {Point} xy
 * @param {Point} zw
 * @returns {Point}
 */
export const addp = ([x, y], [z, w]) => [x + z, y + w]

/**
 * @param {Point} xy
 * @param {number} dx
 * @returns {Point}
 */
export const addx = ([x, y], dx) => [x + dx, y]

/**
 * @param {Point} xy
 * @param {number} dy
 * @returns {Point}
 */
export const addy = ([x, y], dy) => [x, y + dy]

/**
 * @param {Point} xy
 * @returns {Point}
 */
export const negp = ([x, y]) => [-x, -y]
