/**
 * @param {number} n
 * @return {number}
 */

let cache = [];
var climbStairs = function(n) {
    if(n <= 3){
        cache.push(n);
        return n;
    }
    if(!cache[n]){
        cache[n] = climbStairs(n - 1) + climbStairs(n - 2);
    }
    return cache[n];
}