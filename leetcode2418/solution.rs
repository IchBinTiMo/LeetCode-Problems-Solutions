/*
Solution 1:

Time: O(nlog(n)) | Space: O(n)

- n: length of names
 */

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people = names.into_iter().zip(heights.into_iter()).collect::<Vec<(String, i32)>>();

        people.sort_unstable_by_key(|p| -p.1);

        people.into_iter().map(|p| p.0).collect::<Vec<String>>()
    }
}