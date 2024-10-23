use crate::ksliding::get_kmers;
use crate::vector::vec_skip_duplicates;

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

pub fn jaccard_similarity_via_kmers(src1: &str, src2: &str, k: usize) -> f32 {
    let slides1 = get_kmers(src1, k);
    let slides2 = get_kmers(src2, k);
    // print_vec(&slides1);
    // print_vec(&slides2);

    // Maybe it's better to remove duplicates in-place?
    let unique_slides1 = vec_skip_duplicates(slides1);
    let unique_slides2 = vec_skip_duplicates(slides2);
    // print_vec(&unique_slides1);
    // print_vec(&unique_slides2);

    jaccard_similarity(&unique_slides1, &unique_slides2)
}
