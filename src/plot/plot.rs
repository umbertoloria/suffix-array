use crate::plot::interface::{BarPlot, GroupOfBars, SingleBar, SingleBarRectangle};
use crate::suffix_array::monitor::{ExecutionInfo, ExecutionTiming};
use plotters::prelude::full_palette::{
    BLUE_400, GREEN_500, GREY, GREY_500, GREY_600, GREY_700, ORANGE_300, ORANGE_500, ORANGE_600,
    PURPLE, RED_600, YELLOW_600,
};
use plotters::prelude::{GREEN, RED};

struct ExecutionGenerics {
    chunk_size: u32,
    execution_timing: ExecutionTiming,
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
}
impl ExecutionGenerics {
    pub fn create_record(&self) -> Vec<u32> {
        vec![
            self.chunk_size,
            self.execution_timing.whole_duration.as_millis() as u32,
            self.param_1,
            self.param_2,
            self.param_3,
            self.param_4,
        ]
    }
}

pub fn draw_plot_from_monitor(
    fasta_file_name: &str,
    chunk_size_and_execution_info_list: Vec<(usize, ExecutionInfo)>,
) {
    // Duration Composite + Chunk Size + Duration + 4 monitor parameters.
    let num_cols_per_data_item: u32 = 1 + 2 + 4;

    let mut groups_of_bars = Vec::new();

    let mut colors_for_partial_durations = vec![
        GREY_500,   // execution_timing.prop_p11,
        GREY_600,   // execution_timing.prop_p12,
        ORANGE_300, // execution_timing.prop_p21,
        RED_600,    // execution_timing.prop_p22,
        ORANGE_600, // execution_timing.prop_p23,
        GREEN_500,  // execution_timing.prop_p24,
        YELLOW_600, // execution_timing.prop_p3,
        GREY_700,   // execution_timing.prop_extra,
    ];
    let execution_generics_list = chunk_size_and_execution_info_list
        .into_iter()
        .map(|(chunk_size, execution_info)| {
            let (execution_timing, execution_outcome) = execution_info;
            let chunk_size = chunk_size as u32;
            ExecutionGenerics {
                chunk_size,
                execution_timing,
                param_1: execution_outcome.compares_using_rules as u32,
                param_2: execution_outcome.compares_using_strcmp as u32,
                param_3: execution_outcome.compares_with_one_cf as u32,
                param_4: execution_outcome.compares_with_two_cfs as u32,
            }
        })
        .collect::<Vec<_>>();

    let mut records = Vec::new();
    for monitor_value_for_chunk_size in &execution_generics_list {
        let record = monitor_value_for_chunk_size.create_record();
        records.push(record);
    }
    let min_x = execution_generics_list
        .first()
        .expect("Data List should no be empty")
        .chunk_size;
    let max_x = execution_generics_list.last().unwrap().chunk_size;

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
    while i < execution_generics_list.len() {
        let monitor_value_for_chunk_size = &execution_generics_list[i];
        let record = &records[i];
        let chunk_size = record[0];
        let mut group_of_bars = GroupOfBars::new();

        // Composite Bar
        let mut composite_bar = SingleBar::new();
        {
            let mut curr_y_bottom = min_height;

            let execution_timing = &monitor_value_for_chunk_size.execution_timing;
            let props = vec![
                execution_timing.prop_p11,
                execution_timing.prop_p12,
                execution_timing.prop_p21,
                execution_timing.prop_p22,
                execution_timing.prop_p23,
                execution_timing.prop_p24,
                execution_timing.prop_p3,
                execution_timing.prop_extra,
            ];
            let mut j = 0;
            for prop in props {
                let proportional_value = // Value "min_height" is not included.
                    (prop * (leeway_height_for_displaying_values as f64)) as i32;
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
