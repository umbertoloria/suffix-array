pub fn cfl(s: &str) -> Vec<String> {
    // NOTE: Works using chars as bytes.
    let factors = cfl_duval(s.as_bytes());

    let mut result = Vec::new();
    for i in 0..factors.len() {
        let factor_bytes = factors[i];
        let factor = String::from_utf8(factor_bytes.to_vec()).unwrap();
        result.push(factor);
    }

    result
}

pub fn cfl_duval(s: &[u8]) -> Vec<&[u8]> {
    let mut res = Vec::new();
    let n = s.len();
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        let mut k = i;
        while j < n && s[k] <= s[j] {
            if s[k] < s[j] {
                k = i;
            } else {
                k += 1;
            }
            j += 1;
        }
        let new_factor_bytes = &s[i..i + j - k];
        // let new_factor_str = String::from_utf8(new_factor_bytes.to_vec()).unwrap();
        // res.push(&new_factor_str);
        res.push(new_factor_bytes);
        i += j - k;
    }
    res
}
