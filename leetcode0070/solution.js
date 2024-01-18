/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function(n) {
    let prev = 1;
    let current = 1;

    for(let i = 2; i <= n; i++) {
        let tmp = current;
        current = prev + current;
        prev = tmp;
    }

    return current;
};

// /**
//  * @param {number} n
//  * @return {number}
//  */

// let cache = [];
// var climbStairs = function(n) {
//     if(n <= 3){
//         cache.push(n);
//         return n;
//     }
//     if(!cache[n]){
//         cache[n] = climbStairs(n - 1) + climbStairs(n - 2);
//     }
//     return cache[n];
// }