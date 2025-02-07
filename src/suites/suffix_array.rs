use crate::suffix_array::new_suffix_array::DebugMode;
use crate::suffix_array::suite::suite_complete_on_fasta_file;

pub fn main_suffix_array() {
    // let chunk_size_interval = (3, 35);
    let chunk_size_interval = (15, 50);
    // let chunk_size_interval = (3, 6);
    // let chunk_size_interval = (3, 3);
    let chunk_size_interval = (1, 50);
    let chunk_size_interval = (5, 22);
    // let chunk_size_interval = (4, 20);
    // let chunk_size_interval = (3, 50);

    // let perform_logging = true;
    let perform_logging = false;

    // let debug_mode = DebugMode::Verbose;
    // let debug_mode = DebugMode::Overview;
    let debug_mode = DebugMode::Silent;

    // suite_complete_on_fasta_file("000", chunk_size_interval, perform_logging, debug_mode);
    // suite_complete_on_fasta_file("001", chunk_size_interval, perform_logging, debug_mode);
    // suite_complete_on_fasta_file("002_mini", chunk_size_interval, perform_logging, debug_mode);
    // suite_complete_on_fasta_file("002_70", chunk_size_interval, perform_logging, debug_mode);
    suite_complete_on_fasta_file("002_700", chunk_size_interval, perform_logging, debug_mode);
    // suite_complete_on_fasta_file("002_7000", chunk_size_interval, perform_logging, debug_mode);
    // suite_complete_on_fasta_file("002_70000", chunk_size_interval, perform_logging, debug_mode);
}
