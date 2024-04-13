impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        
        heights.insert(0, 0);
        heights.push(0);

        let mut stack: Vec<usize> = vec![0];

        let mut res: i32 = 0;

        for (idx, &height) in heights.iter().enumerate() {
            let mut left: usize = *stack.last().unwrap();

            while height < heights[left] {
                let block_height: i32 = heights[stack.pop().unwrap()];
                left = *stack.last().unwrap();

                let block_width: i32 = (idx - left - 1) as i32;

                res = res.max(block_height * block_width);
            }

            stack.push(idx);
        }

        res
    }
}



// impl Solution {
//     pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//         let mut stack: Vec<usize> = Vec::new();
//         let mut max_area: i32 = 0;
//         for i in 0..=heights.len() {
//             while !stack.is_empty() && (i == heights.len() || heights[i] < heights[*stack.last().unwrap()]) {
//                 let height = heights[stack.pop().unwrap()];
//                 let width = if stack.is_empty() {
//                     i as i32
//                 } else {
//                     (i - *stack.last().unwrap() - 1) as i32
//                 };
//                 max_area = max_area.max(height * width);
//             }
//             stack.push(i);
//         }
//         max_area
//     }
// }

