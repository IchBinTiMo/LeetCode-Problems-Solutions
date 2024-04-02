/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isIsomorphic = function(s, t) {
    let comp1 = {};
    let comp2 = {};
    for(let i = 0; i < s.length; i++){
        if(comp1[s[i]] === undefined && comp2[t[i]] === undefined){
            comp1[s[i]] = t[i];
            comp2[t[i]] = s[i];
            continue;
        }
        if(comp1[s[i]] !== t[i]){
            return false;
        }
        if(comp2[t[i]] !== s[i]){
            return false;
        }

    }
    
    return true;
};