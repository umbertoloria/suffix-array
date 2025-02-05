pub fn get_path_in_generated_folder(filename: &str) -> String {
    format!("generated/{}.fasta", filename)
}

pub fn get_path_in_logged_folder_prefix_trie(filename: &str, chunk_size: usize) -> String {
    format!("logged/{}-{}-prefix-trie.txt", filename, chunk_size)
}

pub fn get_path_in_logged_folder_prefix_tree(filename: &str, chunk_size: usize) -> String {
    format!("logged/{}-{}-prefix-tree.txt", filename, chunk_size)
}
