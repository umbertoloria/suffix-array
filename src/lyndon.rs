use std::i32;

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

/// ICFL
pub fn icfl(s: &str) -> Vec<String> {
    // NOTE: Works using chars as bytes.
    let factors = icfl_bytes(s.as_bytes());

    let mut result = Vec::new();
    for i in 0..factors.len() {
        let factor_bytes = &factors[i];
        let factor = String::from_utf8(factor_bytes.to_vec()).unwrap();
        result.push(factor);
    }

    result
}
pub fn icfl_bytes(w: &[u8]) -> Vec<Vec<u8>> {
    let (x, y) = icfl_find_prefix(w);

    // if x == w + '0' // Should be.
    if x.len() == w.len() + 1 && x[w.len()] == '0'.try_into().unwrap() {
        let mut i = 0;
        while i < w.len() && x[i] == w[i] { i += 1; }
        if i == w.len() {
            return [w.to_vec()].to_vec();
        }
    }
    let (p, bre, last) = icfl_find_bre(&x, &y);

    // l = icfl(bre + y) // Should be.
    let mut bre_plus_y = bre.clone();
    bre_plus_y.extend(y);
    let mut l = icfl_bytes(bre_plus_y.as_slice());
    if l[0].len() > last.try_into().unwrap() { // |m1'| > |r|
        l.insert(0, p);
    } else {
        // l[0] = p + l[0]; // Should be.
        for i in 0..p.len() {
            l[0].insert(0, p[p.len() - 1 - i]);
        }
    }
    l
}

pub fn icfl_find_prefix(w: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let n = w.len();
    if n == 1 {
        let mut new_w = w.to_vec();
        new_w.push('0'.try_into().unwrap());
        // return (w + '0', '');
        return (new_w, Vec::new());
    }

    let mut i = 0;
    let mut j = 1;
    while j < n - 1 && w[j] <= w[i] {
        if w[j] < w[i] {
            i = 0;
        } else {
            i += 1;
        }
        j += 1;
    }

    if j == n - 1 {
        if w[j] <= w[i] {
            let mut new_w = w.to_vec();
            new_w.push('0'.try_into().unwrap());
            // return (w + '0', '');
            return (new_w, Vec::new());
        }
    }

    let mut res1 = Vec::new();
    for i in 0..j + 1 { res1.push(w[i]); }
    let mut res2 = Vec::new();
    for i in j + 1..w.len() { res2.push(w[i]); }
    // return (w[:j+1], w[j+1:])
    (res1, res2)
}

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
