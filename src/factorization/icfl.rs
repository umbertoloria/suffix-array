use std::i32;

pub fn get_icfl_indexes(s_bytes: &[u8]) -> Vec<usize> {
    // NOTE: Works using chars as bytes.
    let icfl_factors_in_bytes = icfl_bytes(s_bytes);
    let mut result = Vec::with_capacity(icfl_factors_in_bytes.len());
    let mut i = 0;
    // Reference so that it will be freed after.
    for factor_bytes in &icfl_factors_in_bytes {
        result.push(i);
        i += factor_bytes.len();
    }
    result
}

pub fn icfl(s: &str) -> Vec<String> {
    // NOTE: Works using chars as bytes.
    let mut result = Vec::new();

    let factors_in_bytes = icfl_bytes(s.as_bytes());
    for factor_bytes in factors_in_bytes {
        let factor = String::from_utf8(factor_bytes).unwrap();
        result.push(factor);
    }

    result
}

const ZERO_SYMBOL_BYTE: u8 = '0' as u8;
fn icfl_bytes(w: &[u8]) -> Vec<Vec<u8>> {
    /**
    input: a string w
    output: the inverse factorization of w obtained with the algorithm ICFL
        If w is an inverse lyndon word, ICFL(w) = w otherwise we have w=pv
        and ICFL(v) = (m1', ..., mk') and p' bounded right extension of p in w.
        ICFL(w) = (p) + ICFL(v)         if p' = rb <= m1'
                  (pm1', m2', ..., mk') if m1' <= r
    */
    let (x, y) = icfl_find_prefix(w);

    // if x == w + '0' // Should be.
    if x.len() == w.len() + 1 && x[w.len()] == ZERO_SYMBOL_BYTE {
        let mut i = 0;
        while i < w.len() && x[i] == w[i] {
            i += 1;
        }
        if i == w.len() {
            return vec![w.to_vec()];
        }
    }
    let (p, bre, last) = icfl_find_bre(&x, &y);

    // l = icfl(bre + y); // Should be.
    let mut bre_plus_y = bre;
    bre_plus_y.extend(y);
    let mut l = icfl_bytes(&bre_plus_y);
    if l[0].len() as i32 > last {
        // |m1'| > |r|
        l.insert(0, p);
    } else {
        // l[0] = p + l[0]; // Should be.
        for i in 0..p.len() {
            l[0].insert(0, p[p.len() - 1 - i]);
        }
    }
    l
}

fn icfl_find_prefix(w: &[u8]) -> (Vec<u8>, Vec<u8>) {
    /**
    input: a string w
    output: (x, y) where x = w0, y = '' if w in an inverse Lyndon word
        w = xy, x = pp' where (p, p') ∈ Pref_bre(w), otherwise.
        p is an inverse Lyndon word which is a proper prefix of w = pv;
        p' is the bounded right extension of p in w.
        A bounder right extension is a proper prefix of v such that:
            - p' is an inverse Lyndon word
            - pz' is an inverse Lyndon word for each proper prefix z' of p'
            - pp' is not an inverse Lyndon word
            - p << p' (p < p' and p is not a proper prefix of p')
        Pref_bre(w) = {(p, p') | p is an inverse Lyndon word which is a non
            empty proper prefix of w }
    */
    let n = w.len();
    if n == 1 {
        // return (w + '0', '');
        let mut new_w = w.to_vec();
        new_w.push(ZERO_SYMBOL_BYTE);
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
            // return (w + '0', '');
            let mut new_w = w.to_vec();
            new_w.push(ZERO_SYMBOL_BYTE);
            return (new_w, Vec::new());
        }
    }

    (
        //
        (&w[0..j + 1]).to_vec(),
        (&w[j + 1..]).to_vec(),
    )
}

fn icfl_find_bre(x: &[u8], y: &[u8]) -> (Vec<u8>, Vec<u8>, i32) {
    /**
    input: (x, y) where w = xy is not an inverse Lyndon word;
        x = pp' = raurb, (p, p') ∈ Pref_bre(w)
    output: (p, p', y, last) = (rau, rb, y, |r|)
    */
    let mut w = Vec::with_capacity(x.len() + y.len());
    w.extend_from_slice(x);
    w.extend_from_slice(y);

    let n = x.len() as i32 - 1;
    let f = icfl_get_failure_function(x, x.len() - 1); // Border(raur)

    let mut i = n - 1;
    let mut last = n;

    while i >= 0 {
        let i_usize = i as usize;
        if w[f[i_usize]] < x[x.len() - 1] {
            last = f[i_usize] as i32 - 1;
        }
        i = f[i_usize] as i32 - 1;
    }

    let mut sep1_i32 = n - last - 1;
    if sep1_i32 < 0 {
        sep1_i32 += w.len() as i32;
    }
    let sep1_usize = sep1_i32 as usize;
    let sep2_usize = (n + 1) as usize;

    let res1 = (&w[0..sep1_usize]).to_vec();
    let res2 = if sep2_usize > sep1_usize {
        (&w[sep1_usize..sep2_usize]).to_vec()
    } else {
        Vec::new()
    };

    (res1, res2, last + 1)
}

fn icfl_get_failure_function(s: &[u8], s_inner_size: usize) -> Vec<usize> {
    // Here we fake that "m" is the size of "s", since the caller is most likely to exclude the last
    // item of "s".
    // let m = s.len();
    let m = s_inner_size;

    let mut f = Vec::with_capacity(m);
    for _ in 0..m {
        f.push(0);
    }

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
