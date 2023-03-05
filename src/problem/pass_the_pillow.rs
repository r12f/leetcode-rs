#[solution]
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let rounds = (time / (n - 1));

        // Return pass
        if rounds % 2 == 1 {
            n - time % (n - 1)
        } else {
            time % (n - 1) + 1
        }
    }
}
