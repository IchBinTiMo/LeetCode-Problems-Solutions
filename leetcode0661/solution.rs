impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        let m = img.len();
        let n = img[0].len();


        for i in 0..m {
            ans.push(Vec::new());

            for j in 0..n {
                let mut cnt = 0;
                let mut acc = 0;

                for row in 0..3 {
                    for col in 0..3 {
                        match i + row - 1  > m - 1 || j + col - 1 > n - 1 {
                            true => {},
                            _ => {
                                cnt += 1;
                                acc += img[i + row - 1][j + col - 1];
                            }
                        }
                    }
                }

                ans[i].push(acc / cnt);
            }
        }

        ans
    }
}