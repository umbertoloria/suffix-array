pub fn jaccard_similarity(a: &Vec<&str>, b: &Vec<&str>) -> f32 {
    // Assuming no duplicates in "a" and "b".
    let intersection = count_shared_items(a, b);
    let union = a.len() + b.len() - intersection;
    intersection as f32 / union as f32
}

fn count_shared_items(vec1: &Vec<&str>, vec2: &Vec<&str>) -> usize {
    // Sorted before calling this? We'll see...
    let mut result = 0;
    for item1 in vec1 {
        if vec2.contains(item1) {
            result += 1;
            continue;
        }
    }
    result
}
