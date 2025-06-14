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

pub fn get_path_for_project_factorization_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-a-fact.txt",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}

pub fn get_path_for_project_tree_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-a-tree.txt",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}

pub fn get_path_for_project_full_tree_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-aa-full_tree.txt",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}

pub fn get_path_for_project_mini_tree_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-aa-mini-tree.txt",
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

pub fn get_path_for_project_prefix_tree_add_file(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-c-prefix-tree-add.txt",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}

pub fn get_path_for_project_outcome_file_json(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-za-execution.json",
        get_path_for_project_folder(filename),
        filename,
        chunk_size
    )
}

pub fn get_path_for_project_timing_file_json(filename: &str, chunk_size: usize) -> String {
    format!(
        "{}/{}-{}-zb-timing.json",
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
