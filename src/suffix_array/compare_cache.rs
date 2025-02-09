use std::collections::HashMap;

pub struct CompareCache {
    pub cache: HashMap<(usize, usize), bool>,
}
impl CompareCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    pub fn compare_1_before_2(&mut self, str: &str, ls_index_1: usize, ls_index_2: usize) -> bool {
        if ls_index_1 == ls_index_2 {
            return false; // Not before.
        }

        if ls_index_1 > ls_index_2 {
            !Self::perform_compare(str, ls_index_2, ls_index_1)
        } else {
            Self::perform_compare(str, ls_index_1, ls_index_2)
        }
        /*
        // TODO: Optimize and enable this cache
        if let Some(&result) = self.cache.get(&(ls_index_1, ls_index_2)) {
            // println!(" -> comparing {} with {}", ls_index_1, ls_index_2);
            result
        } else {
            let cmp1 = &str[ls_index_1..];
            let cmp2 = &str[ls_index_2..];
            // println!(" -> **************** comparing {} with {}", ls_index_1, ls_index_2);
            let result = if cmp1 < cmp2 { true } else { false };
            self.cache.insert((ls_index_1, ls_index_2), result);
            result
        }
        */
    }
    fn perform_compare(str: &str, ls_index_1: usize, ls_index_2: usize) -> bool {
        // println!(" -> *** comparing {} with {}", ls_index_1, ls_index_2);
        let cmp1 = &str[ls_index_1..];
        let cmp2 = &str[ls_index_2..];
        if cmp1 < cmp2 {
            true
        } else {
            false
        }
    }
}
