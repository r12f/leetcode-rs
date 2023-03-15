struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in strs {
            result.push_str(&s);
            result.push('\0');
        }

        result
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut result = Vec::new();

        let bytes = s.as_bytes();

        let mut left = 0;
        while left < bytes.len() {
            let mut right = left;
            while bytes[right] != '\0' as u8 {
                right += 1;
            }

            if left == right {
                result.push("".to_string());
            } else {
                let mut buffer = vec![0 as u8; right - left];
                buffer.clone_from_slice(&bytes[left..right]);

                let mut parsed = unsafe { String::from_utf8_unchecked(buffer) };
                result.push(parsed);
            }

            left = right + 1;
        }

        result
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use lcrt::lc_strvec;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_271() {
        let strs = lc_strvec!["Hello", "World"];

        let obj = Codec::new();
        let s: String = obj.encode(strs);
        let ans: Vec<String> = obj.decode(s);
        assert_eq!(lc_strvec!["Hello", "World"], ans);
    }
}
