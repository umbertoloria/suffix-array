use std::i32;

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
pub fn icfl_find_bre(x: &[u8], y: &[u8]) -> (Vec<u8>, Vec<u8>, i32) {
    // TODO: Improve conversion logics
    let mut w = Vec::with_capacity(x.len() + y.len());
    for i in 0..x.len() { w.push(x[i]); }
    for i in 0..y.len() { w.push(y[i]); }
    let w = w.as_slice();

    let n = i32::try_from(x.len() - 1).unwrap();
    let f = icfl_get_failure_function(x, x.len() - 1);
    let f = f.as_slice();

    let mut i = n - 1;
    let mut last = n;

    while i >= 0 {
        if w[f[i as usize]] < x[x.len() - 1] {
            last = i32::try_from(f[i as usize]).unwrap() - 1;
        }
        // i = f[i] - 1; // Should be.
        i = i32::try_from(f[i as usize]).unwrap() - 1;
    }

    let mut first_separator_i32 = n - last - 1;
    if first_separator_i32 < 0 {
        first_separator_i32 += i32::try_from(w.len()).unwrap();
    }
    let sep1_usize = usize::try_from(first_separator_i32).unwrap();

    let sep2_i32 = n + 1;
    let sep2_usize = usize::try_from(sep2_i32).unwrap();

    let mut res1 = Vec::with_capacity(sep1_usize);
    let mut res2 = Vec::with_capacity(
        usize::try_from(
            i32::max(
                i32::try_from(sep2_usize).unwrap()
                    - i32::try_from(sep1_usize).unwrap(),
                0,
            ),
        ).unwrap(),
    );
    for i in 0..sep1_usize { res1.push(w[i]); }
    for i in sep1_usize..sep2_usize { res2.push(w[i]); }

    (res1, res2, last + 1)
}

pub fn icfl_get_failure_function(s: &[u8], s_inner_size: usize) -> Vec<usize> {
    // Here we fake that "m" is the size of "s", since the caller is most likely to exclude the last
    // item of "s".
    // let m = s.len();
    let m = s_inner_size;

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
