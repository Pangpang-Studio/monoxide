/**
 * Returns the weighted average of two values.
 *
 * @param {number} z
 * @param {number} w
 * @param {number} ratio The weighting factor of z
 * @returns {number} The weighted average of z and w
 */
export function mix(z, w, ratio) {
  return w + ratio * (z - w)
}
