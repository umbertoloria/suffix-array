pub fn get_path_in_generated_folder(filename: &str) -> String {
    format!("generated/{}.fasta", filename)
}

pub fn get_path_for_project_folder(filename: &str) -> String {
    format!("results/{}", filename)
}

pub fn get_path_for_project_prefix_trie_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-a-prefix-trie.txt",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}

pub fn get_path_for_project_prefix_tree_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-b-prefix-tree.txt",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}

pub fn get_path_for_project_suffix_array_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-z-suffix-array.txt",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}
