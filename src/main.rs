mod lyndon;

fn main() {
    let src = "umberto";
    let factors = lyndon::cfl_duval(src);

    for factor_byte in factors {
        let factor_str = String::from_utf8(factor_byte.to_vec()).unwrap();
        println!("{}", factor_str);
    }
}
