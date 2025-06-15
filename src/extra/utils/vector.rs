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

pub fn print_array_of_numbers(list: &Vec<usize>) -> String {
    let mut result = String::from("[");

    let last_item = list[list.len() - 1];
    for i in 0..list.len() - 1 {
        let curr_item = list[i];
        result.push_str(&format!("{}, ", curr_item));
    }
    result.push_str(&format!("{}]", last_item));

    result
}
