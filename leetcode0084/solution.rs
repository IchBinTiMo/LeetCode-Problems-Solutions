impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area: i32 = 0;
        for i in 0..=heights.len() {
            while !stack.is_empty() && (i == heights.len() || heights[i] < heights[*stack.last().unwrap()]) {
                let height = heights[stack.pop().unwrap()];
                let width = if stack.is_empty() {
                    i as i32
                } else {
                    (i - *stack.last().unwrap() - 1) as i32
                };
                max_area = max_area.max(height * width);
            }
            stack.push(i);
        }
        max_area
    }
}

