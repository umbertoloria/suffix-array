#![allow(warnings)]

use crate::suffix_array::suite::suite_complete_on_fasta_file;

mod extra;
mod factorization;
mod files;
mod plot;
mod suffix_array;
mod suites;
mod utils;

fn main() {
    // TODO: Control this main with CLI Interface with Arguments
    // OLD SUITES
    // main_generation();
    // main_factorization();

    // Chunk Size Interval
    let chunk_size_vec_000 = create_chunk_size_interval_and_none(2, 7);
    let chunk_size_vec_001 = create_chunk_size_interval_and_none(2, 8);
    let chunk_size_vec_002_mini = create_chunk_size_interval_and_none(2, 24);
    let chunk_size_vec = create_chunk_size_interval(1, 50);
    let chunk_size_vec_70 = merge_chunk_size_intervals(
        create_chunk_size_interval(2, 9),
        merge_chunk_size_intervals(
            create_chunk_size_of_steps(10, 100, 10),
            merge_chunk_size_intervals(
                create_chunk_size_of_steps(100, 1_000, 100),
                merge_chunk_size_intervals(
                    create_chunk_size_of_steps(1_000, 10_000, 1_000),
                    merge_chunk_size_intervals(
                        create_chunk_size_of_steps(10_000, 56_000, 1000),
                        vec![Some(56_000), Some(56_137), None],
                    ),
                ),
            ),
        ),
    );
    let chunk_size_vec_700_1 = merge_chunk_size_intervals(
        create_chunk_size_interval(4, 9),
        merge_chunk_size_intervals(
            create_chunk_size_of_steps(10, 100, 10),
            vec![Some(100), Some(500), Some(1_000)],
        ),
    );
    let chunk_size_vec_700_2 = merge_chunk_size_intervals(
        vec![Some(2_000), Some(5_000)],
        merge_chunk_size_intervals(
            vec![Some(10_000), Some(50_000)],
            merge_chunk_size_intervals(
                create_chunk_size_of_steps(100_000, 500_000, 100_000),
                vec![Some(500_000), Some(598_865), None],
            ),
        ),
    );
    // let chunk_size_vec = create_chunk_size_interval(5, 200);
    // let chunk_size_vec = create_chunk_size_interval(5, 30);
    // let chunk_size_vec = create_chunk_size_interval(5, 5);
    // let chunk_size_vec = create_chunk_size_of_thousands_with_steps(1, 70);
    // let chunk_size_none_list = vec![None];

    // Logging?
    let le = true;
    let lf = false;
    // let lf = true;
    let lts = false;
    // let lts = true;

    // suite_complete_on_fasta_file("000", &chunk_size_vec_000, 25, le, lf, lts);
    // suite_complete_on_fasta_file("001", &chunk_size_vec_001, 25, le, lf, lts);
    // suite_complete_on_fasta_file("002_mini", &chunk_size_vec_002_mini, 30, le, lf, lts);
    // suite_complete_on_fasta_file("002_70", &chunk_size_vec, 70_000, le, lf, false);
    suite_complete_on_fasta_file("002_70", &chunk_size_vec_70, 200_000, le, lf, false);
    // suite_complete_on_fasta_file("002_700", &chunk_size_vec_700_1, 1_600_000, le, lf, false);
    // suite_complete_on_fasta_file("002_700", &chunk_size_vec_700_2, 30_000_000, le, lf, false);
    // suite_complete_on_fasta_file("002_7000", &chunk_size_vec, 50_000_000, le, lf, false);
}

fn create_chunk_size_interval(min: usize, max: usize) -> Vec<Option<usize>> {
    (min..=max).map(|x| Some(x)).collect()
}

fn create_chunk_size_interval_and_none(min: usize, max: usize) -> Vec<Option<usize>> {
    let mut result = create_chunk_size_interval(min, max);
    result.push(None);
    result
}

fn create_chunk_size_of_steps(min: usize, max_excl: usize, step: usize) -> Vec<Option<usize>> {
    let mut result = Vec::new();
    let mut curr = min;
    while curr < max_excl {
        result.push(Some(curr));
        curr += step;
    }
    result
}

fn merge_chunk_size_intervals(
    a: Vec<Option<usize>>,
    mut b: Vec<Option<usize>>,
) -> Vec<Option<usize>> {
    let mut result = a;
    result.append(&mut b);
    result
}

fn create_chunk_size_of_thousands_with_steps(min: usize, max: usize) -> Vec<Option<usize>> {
    (min..=max)
        .map(|x| (x * 1_000, x * 1_000 + 250, x * 1_000 + 500, x * 1_000 + 750))
        .flat_map(|a| vec![a.0, a.1, a.2, a.3])
        .map(|x| Some(x))
        .collect()
}
