impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        let mut current: usize = 0;

        while current < tokens.len() {

            if let Ok(val) = tokens[current].parse::<i32>() {
                stack.push(val);
            } else {
                let v2: i32 = stack.pop().unwrap();
                let v1: i32 = stack.pop().unwrap();
                match &*tokens[current] {
                    "+" => stack.push(v1 + v2),
                    "-" => stack.push(v1 - v2),
                    "*" => stack.push(v1 * v2),
                    "/" => stack.push(v1 / v2),
                    _ => {}
                }
            }

            current += 1;
        }
        
        *stack.last().unwrap()
    }
}