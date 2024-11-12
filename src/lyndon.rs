pub fn cfl_duval(word: &str) -> Vec<&[u8]> {
    // NOTE: Works using bytes as chars.

    let mut res: Vec<&[u8]> = Vec::new();

    let s = word.as_bytes();

    let n = word.len();
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

/// ICFL
pub fn icfl_get_failure_function(s: &[u8]) -> Vec<usize> {
    let m = s.len();

    let mut f = Vec::with_capacity(m);
    for _ in 0..m { f.push(0); }

    let mut i = 1;
    let mut j = 0;
    while i < m {
        if s[j] == s[i] {
            f[i] = j + 1;
            i += 1;
            j += 1;
        } else if j > 0 {
            j = f[j - 1];
        } else {
            f[i] = 0;
            i += 1;
        }
    }
    f
}
