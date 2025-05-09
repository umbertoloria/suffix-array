use crate::extra::comparing::ksliding::{get_kmers, KmersSet};

pub fn jaccard_similarity(a: KmersSet, b: KmersSet) -> f32 {
    let intersection = a.intersection(&b);
    let intersection_length = intersection.count();
    let union = a.len() + b.len() - intersection_length;
    intersection_length as f32 / union as f32
}

pub fn jaccard_similarity_via_kmers(src1: &str, src2: &str, k: usize) -> f32 {
    let slides1 = get_kmers(src1, k);
    let slides2 = get_kmers(src2, k);
    jaccard_similarity(slides1, slides2)
}

/*
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
*/
