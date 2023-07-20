pub fn is_rgba8(raw: &[u8]) -> bool {
    let mut idx = 3;
    while idx < raw.len() {
        if raw[idx] != 0 {
            return true;
        }
        idx += 4
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_rgba8_test() {
        let raw1 = [0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(false, is_rgba8(&raw1));
        let raw2 = [0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(true, is_rgba8(&raw2));
    }
}
