pub struct Solution;

impl Solution {
    pub fn pass_the_pillow_math(n: i32, mut time: i32) -> i32 {
        let mut chunks = time / (n - 1);

        if chunks % 2 == 0 {
            (time % (n - 1)) + 1
        } else {
            n - (time % (n - 1))
        }
    }

    pub fn pass_the_pillow(n: i32, mut time: i32) -> i32 {
        let mut incrementor = 1;
        let mut current_person = 1;

        while time > 0 {
            if current_person == n {
                incrementor = -1;
            } else if current_person == 1 {
                incrementor = 1;
            }
            current_person += incrementor;
            time -= 1;
        }
        current_person
    }
}
