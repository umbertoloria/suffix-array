use crate::plot::interface::{BarPlot, GroupOfBars, SingleBar, SingleBarRectangle};
use crate::suffix_array::monitor::Monitor;
use plotters::prelude::full_palette::{
    BLUE_400, GREEN_500, GREY, GREY_500, GREY_600, GREY_700, ORANGE_300, ORANGE_500, ORANGE_600,
    PURPLE, RED_600, YELLOW_600,
};
use plotters::prelude::{GREEN, RED};
use std::time::Duration;

struct MonitorValuesForChunkSize {
    chunk_size: u32,
    whole_duration: u32,
    durations: Vec<Duration>,
    value_2: u32,
    value_3: u32,
    value_4: u32,
    value_5: u32,
}
impl MonitorValuesForChunkSize {
    pub fn create_record(&self) -> Vec<u32> {
        vec![
            self.chunk_size,
            self.whole_duration,
            self.value_2,
            self.value_3,
            self.value_4,
            self.value_5,
        ]
    }
}

pub fn draw_plot_from_monitor(
    fasta_file_name: &str,
    chunk_and_monitor_pairs: Vec<(usize, Monitor)>,
) {
    // Duration Composite + Chunk Size + Duration + 4 monitor parameters.
    let num_cols_per_data_item: u32 = 1 + 2 + 4;

    let mut groups_of_bars = Vec::new();

    let mut colors_for_partial_durations = vec![
        GREY_500,   // monitor.get_phase1_1_icfl_factorization_duration(),
        GREY_600,   // monitor.get_phase1_2_custom_factorization_duration(),
        ORANGE_300, // monitor.get_phase2_1_prefix_trie_create_duration(),
        RED_600,    // monitor.get_phase2_2_prefix_trie_merge_rankings_duration(),
        ORANGE_600, // monitor.get_phase2_3_prefix_trie_in_prefix_merge_duration(),
        GREEN_500,  // monitor.get_phase2_4_prefix_tree_create_duration(),
        YELLOW_600, // monitor.get_phase3_suffix_array_compose_duration(),
        GREY_700,   // monitor.get_extra_time_spent(),
    ];
    let monitor_values_for_chunk_size_list = chunk_and_monitor_pairs
        .into_iter()
        .map(|(chunk_size, monitor)| {
            let chunk_size = chunk_size as u32;
            let duration = monitor.get_whole_process_duration_included_extra();
            let value_1 = duration.as_millis() as u32;
            let value_2 = monitor.compares_using_rules as u32;
            let value_3 = monitor.compares_using_strcmp as u32;
            let value_4 = monitor.compares_with_one_cf as u32;
            let value_5 = monitor.compares_with_two_cfs as u32;

            MonitorValuesForChunkSize {
                chunk_size,
                whole_duration: value_1,
                durations: vec![
                    monitor.get_phase1_1_icfl_factorization_duration(),
                    monitor.get_phase1_2_custom_factorization_duration(),
                    monitor.get_phase2_1_prefix_trie_create_duration(),
                    monitor.get_phase2_2_prefix_trie_merge_rankings_duration(),
                    monitor.get_phase2_3_prefix_trie_in_prefix_merge_duration(),
                    monitor.get_phase2_4_prefix_tree_create_duration(),
                    monitor.get_phase3_suffix_array_compose_duration(),
                    monitor.get_extra_time_spent(),
                ],
                value_2,
                value_3,
                value_4,
                value_5,
            }
        })
        .collect::<Vec<_>>();

    let mut records = Vec::new();
    for monitor_value_for_chunk_size in &monitor_values_for_chunk_size_list {
        let record = monitor_value_for_chunk_size.create_record();
        records.push(record);
    }
    let min_x = monitor_values_for_chunk_size_list
        .first()
        .expect("Data List should no be empty")
        .chunk_size;
    let max_x = monitor_values_for_chunk_size_list
        .last()
        .unwrap()
        .chunk_size;

    let mut colors = vec![
        PURPLE,     // Chunk Size
        GREY,       // Duration
        GREEN,      // Compare using rules
        RED,        // Compare using strcmp
        BLUE_400,   // Compare with one Custom Factor
        ORANGE_500, // Compare with two Custom Factor
    ];

    // Min and Max values.
    let first_record = records.get(0).unwrap();
    let mut min_values = vec![
        first_record[0],
        first_record[1],
        first_record[2],
        first_record[3],
        first_record[4],
        first_record[5],
    ];
    let mut max_values = min_values.clone();
    for i in 1..records.len() {
        let record = &records[i];
        for j in 0..record.len() {
            if record[j] < min_values[j] {
                min_values[j] = record[j];
            }
            if record[j] > max_values[j] {
                max_values[j] = record[j];
            }
        }
    }

    let min_height = 300;
    let max_height = 10000;
    let leeway_height_for_displaying_values = max_height - min_height;

    let mut i = 0;
    while i < monitor_values_for_chunk_size_list.len() {
        let monitor_value_for_chunk_size = &monitor_values_for_chunk_size_list[i];
        let record = &records[i];
        let chunk_size = record[0];
        let mut group_of_bars = GroupOfBars::new();

        // Composite Bar
        let mut composite_bar = SingleBar::new();
        {
            let durations = &monitor_value_for_chunk_size.durations;
            let mut sum_durations_micros = 0;
            for duration in durations {
                sum_durations_micros += duration.as_micros();
            }
            let mut j = 0;
            let mut curr_y_bottom = min_height;
            while j < durations.len() {
                let curr_duration = &durations[j];
                let curr_micros = curr_duration.as_micros();

                let percentage = curr_micros as f64 / sum_durations_micros as f64;
                let mut proportional_value = // Value "min_height" is not included.
                    (percentage * (leeway_height_for_displaying_values as f64)) as i32;
                if j == durations.len() - 1 {
                    proportional_value = max_height - curr_y_bottom;
                    // Taking care of occupying all the space. The difference is little. It is:
                    //  * "(max_height - curr_y_bottom) - proportional_value"
                }
                let single_bar_rectangle = SingleBarRectangle::new(
                    chunk_size * num_cols_per_data_item,
                    curr_y_bottom,
                    curr_y_bottom + proportional_value,
                    colors_for_partial_durations[j],
                );
                composite_bar.add_rectangle(single_bar_rectangle);

                curr_y_bottom += proportional_value;
                j += 1;
            }
        }
        group_of_bars.add_bar(composite_bar);

        // Single Bars
        for j in 0..record.len() {
            let min_column = min_values[j];
            let max_column = max_values[j];
            let diff_max_min_column = max_column - min_column;

            let value = record[j];
            let percentage = if diff_max_min_column == 0 {
                // If all data are the same, we use a 50% as default value.
                0.5
            } else {
                (value - min_column) as f64 / diff_max_min_column as f64
            };
            let proportional_value = // Value "min_height" is included.
                min_height + (percentage * (leeway_height_for_displaying_values as f64)) as i32;

            let mut single_bar = SingleBar::new();
            single_bar.add_rectangle(
                //
                SingleBarRectangle::new(
                    chunk_size * num_cols_per_data_item + 1 + (j as u32),
                    0,
                    proportional_value,
                    colors[j],
                ),
            );
            group_of_bars.add_bar(single_bar);
        }
        groups_of_bars.push(group_of_bars);

        i += 1;
    }

    let bar_plot = BarPlot::new(3600, 1400, format!("Prefix Trie: {}", fasta_file_name));
    bar_plot.draw(
        format!("./plots/plot-{}.png", fasta_file_name),
        num_cols_per_data_item,
        min_x,
        max_x,
        max_height,
        groups_of_bars,
    );
}
