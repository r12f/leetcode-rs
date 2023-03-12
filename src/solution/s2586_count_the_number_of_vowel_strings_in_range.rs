#[solution]
impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut count = 0;
        for i in left..=right {
            let w = words[i as usize].as_bytes();
            if Self::is_vowel(w[0] as char) && Self::is_vowel(w[w.len() - 1] as char) {
                count += 1;
            }
        }
        count
    }

    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}
