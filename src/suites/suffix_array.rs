use crate::suffix_array::new_suffix_array::DebugMode;
use crate::suffix_array::suite::suite_complete_on_fasta_file;

pub fn main_suffix_array() {
    // Chunk Size Interval
    let chunk_size_interval = (3, 35);
    // let chunk_size_interval = (15, 50);
    // let chunk_size_interval = (3, 6);
    // let chunk_size_interval = (3, 3);
    let chunk_size_interval = (1, 50);
    // let chunk_size_interval = (5, 22);
    // let chunk_size_interval = (4, 20);
    // let chunk_size_interval = (3, 50);
    // let chunk_size_interval = (35, 35);
    // let chunk_size_interval = (3, 6);
    // let chunk_size_interval = (4, 200);

    // Perform Logging?
    let pl = true;
    // let pl = false;

    // Debug Mode
    // let dm = DebugMode::Verbose;
    // let dm = DebugMode::Overview;
    let dm = DebugMode::Silent;

    // suite_complete_on_fasta_file("000", chunk_size_interval, 25, pl, dm);
    // suite_complete_on_fasta_file("001", chunk_size_interval, 25, pl, dm);
    // suite_complete_on_fasta_file("002_mini", chunk_size_interval, 30, pl, dm);
    suite_complete_on_fasta_file("002_70", chunk_size_interval, 70_000, pl, dm);
    // suite_complete_on_fasta_file("002_700", chunk_size_interval, 2_100_000, pl, dm);
    // suite_complete_on_fasta_file("002_7000", chunk_size_interval, pl, dm);
    // suite_complete_on_fasta_file("002_70000", chunk_size_interval, pl, dm);
}
