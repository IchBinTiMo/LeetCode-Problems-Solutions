/**
 * @param {number} n
 * @param {number[]} left
 * @param {number[]} right
 * @return {number}
 */
var getLastMoment = function(n, left, right) {
    return Math.max(left ? Math.max(...left) : 0, right ? (n - Math.min(...right)) : 0);
};