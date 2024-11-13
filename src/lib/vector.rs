pub fn vec_skip_duplicates(vec: Vec<&str>) -> Vec<&str> {
    let mut result = Vec::new();
    let mut first_position;
    for i in 0..vec.len() {
        first_position = true;
        for j in 0..i {
            if vec[i] == vec[j] {
                first_position = false;
                break;
            }
        }
        if first_position {
            result.push(vec[i]);
        }
    }
    result
}
