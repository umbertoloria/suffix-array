pub fn get_path_in_generated_folder(filename: &str) -> String {
    format!("generated/{}.fasta", filename)
}
