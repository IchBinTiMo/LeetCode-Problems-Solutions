impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == String::from("0") || num2 == String::from("0") {
            return String::from("0");
        }

        let mut n1: Vec<u32> = Vec::new();
        let mut n2: Vec<u32> = Vec::new();

        for &c in num1.as_bytes() {
            let current = c - 48;
            n1.push(current as u32);
        }

        for &c in num2.as_bytes() {
            let current = c - 48;
            n2.push(current as u32);
        }

        let mut n3: Vec<u32> = vec![0; num1.len() + num2.len()];

        for i in 0..n1.len() {
            let m1 = n1[n1.len() - 1 - i];
            for j in 0..n2.len() {
                let m2 = n2[n2.len() - 1 - j];
                let product = m1 * m2;

                let k = n3.len();

                n3[k - 1 - (i + j)] += product % 10;
                n3[k - 1 - (i + j + 1)] += product / 10;

                if n3[k - 1 - (i + j)] > 9 {
                    n3[k - 1 - (i + j + 1)] += n3[k - 1 - (i + j)] / 10;
                    n3[k - 1 - (i + j)] %= 10;
                }
            }
        }

        while n3[0] == 0 {
            n3.remove(0);
        }

        n3.iter().map(|x| std::char::from_digit(*x, 10).unwrap()).collect::<String>()



    }
}