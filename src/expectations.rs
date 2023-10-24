#[derive(Debug, Clone)]
pub struct Expectation {
    pub number_of_pairs: i32,
    pub expectation_value: f64,
    pub expectations: Vec<Vec<f64>>,
}

impl Expectation {
    pub async fn calculate(number_of_pairs: i32) -> Self {
        if number_of_pairs < 1 {
            return Expectation {
                number_of_pairs,
                expectation_value: 0f64,
                expectations: vec![vec![0f64]],
            };
        }

        let mut expectations: Vec<Vec<f64>> = Vec::with_capacity(number_of_pairs as usize + 1);

        for n in 0..=number_of_pairs {
            let mut row: Vec<f64> = Vec::with_capacity(number_of_pairs as usize + 1);

            // fill row with 0f64
            for _ in 0..=number_of_pairs {
                row.push(0f64);
            }
            expectations.push(row);

            let mut k = n;
            while k > -1 {
                let kf64 = k as f64;
                let nf64 = n as f64;

                let res = 
                // term1
                ( (kf64 / ((2f64 * nf64) - kf64)) * (1f64 + Self::get_expectation(n - 1, k - 1, &expectations)))
                // term2
                    + ((2f64 * (nf64 - kf64)) / ( (2f64 * nf64) - kf64))
                        * (((1f64 / ((2f64 * nf64) - kf64 - 1f64)) * (1f64 + Self::get_expectation(n - 1, k, &expectations)))
                // term3
                + ((kf64 / ((2f64 * nf64) - kf64 - 1f64)) * (2f64 + Self::get_expectation(n - 1, k, &expectations)))
                // term4
                + (((2f64 * (nf64 - kf64 - 1f64)) /(2f64 * nf64 - kf64 - 1f64)) * (1f64 + Self::get_expectation(n , k + 2, &expectations) )));

                expectations[n as usize][k as usize] = res;

                k -= 1;
            }
        }

        expectations[1][1] = 1f64; // fix for n = 1, k = 1

        Expectation {
            number_of_pairs,
            expectation_value: expectations[number_of_pairs as usize][0],
            expectations,
        }
    }

    fn get_expectation(n: i32, k: i32, expectations: &Vec<Vec<f64>>) -> f64 {
        if k < 0 {
            return 0f64;
        }

        if n == 1 {
            return 1f64;
        }
        if n == k {
            return k.into();
        }
        if k > n {
            return 0f64;
        }

        expectations[n as usize][k as usize]
    }

    pub async fn get_csv(&self) -> String {
        let mut csv = String::new();
        let mut n = 0;
        for row in self.expectations.iter() {
            match n {
                0 => {
                    csv.push_str("n/k,");
                    for k in 0..row.len() {
                        csv.push_str(&format!("{},", k));
                    }
                }
                _ => {
                    csv.push_str(&format!("{},", n));
                    for value in row.iter() {
                        csv.push_str(&format!("{},", value));
                    }
                }
            }
            csv.pop();
            csv.push_str("\n");
            n += 1;
        }
        csv
    }
}
