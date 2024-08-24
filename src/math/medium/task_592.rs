pub struct Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut s = Vec::new();

        let mut t = String::new();
        let mut c = (1, 0, 1);

        for l in expression.chars() {
            if l == '-' || l == '+' {
                if !t.is_empty() {
                    let n = t.parse::<i32>().unwrap();
                    c.2 = n;
                    t = String::new();
                }

                s.push(c);

                if l == '-' {
                    c = (-1, 0, 1);
                } else {
                    c = (1, 0, 1)
                }
            } else if l == '/' {
                let n = t.parse::<i32>().unwrap();

                c.1 = n;

                t = String::new();
            } else {
                t.push(l);
            }
        }

        let n = t.parse::<i32>().unwrap();

        c.2 = n;

        s.push(c);

        let mut t = (1, 0, 1);

        for &c in &s {
            let a = t.0 * t.1 * c.2;
            let b = c.0 * c.1 * t.2;
            let d = c.2 * t.2;
            let mut n = a + b;

            if n == 0 {
                t = (1, 0, 1);
            } else {
                if n < 0 {
                    t.0 = -1;
                    n = n.abs();
                } else {
                    t.0 = 1;
                }

                let g = if n >= d {Self::gcd(n, d)} else {Self::gcd(d, n)};

                t.1 = n / g;
                t.2 = d / g;
            }
        }

        let mut r = String::new();

        if t.0 == -1 {
            r.push('-');
        }

        r.push_str(&t.1.to_string());
        r.push('/');
        r.push_str(&t.2.to_string());

        r
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        if a % b == 0 {b} else {Self::gcd(b, a % b)}
    }
}