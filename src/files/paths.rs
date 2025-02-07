pub fn get_path_in_generated_folder(filename: &str) -> String {
    format!("generated/{}.fasta", filename)
}

pub fn get_path_for_project_folder(filename: &str) -> String {
    format!("results/{}", filename)
}
pub fn get_path_for_plots_folder(filename: &str) -> String {
    format!("plots/{}", filename)
}
pub fn get_path_for_plot_file(
    fasta_file_name: &str,
    min_chunk_size: usize,
    max_chunk_size: usize,
) -> String {
    get_path_for_plots_folder(&format!(
        "plot-{}-chunks-{}-{}.png",
        fasta_file_name, min_chunk_size, max_chunk_size
    ))
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

pub fn get_path_for_project_monitor_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-zz-monitor.txt",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}
