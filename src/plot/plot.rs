use crate::files::paths::get_path_for_plot_file;
use crate::plot::interface::{BarPlot, GroupOfBars, SingleBar, SingleBarRectangle};
use crate::suffix_array::monitor::{ExecutionInfo, ExecutionTiming};
use plotters::prelude::full_palette::{
    BLUE_400, GREEN_500, GREY, GREY_500, GREY_600, GREY_700, ORANGE_300, ORANGE_500, ORANGE_600,
    PURPLE, RED_600, YELLOW_600,
};
use plotters::prelude::{GREEN, RED};

pub fn draw_plot_from_monitor(
    fasta_file_name: &str,
    chunk_size_and_execution_info_list: Vec<(usize, ExecutionInfo)>,
) {
    // Duration Composite + Chunk Size + Duration + 4 monitor parameters + gap.
    let num_cols_per_data_item: u32 = 1 + 2 + 4 + 1;

    let min_chunk_size = chunk_size_and_execution_info_list
        .first()
        .expect("List should no be empty")
        .0;
    let max_chunk_size = chunk_size_and_execution_info_list.last().unwrap().0;

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

    struct ExecutionGenerics {
        chunk_size: u32,
        execution_timing: ExecutionTiming,
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
    }

    let execution_generics_list = chunk_size_and_execution_info_list
        .into_iter()
        .map(|(chunk_size, execution_info)| {
            let execution_timing = execution_info.execution_timing;
            let execution_outcome = execution_info.execution_outcome;
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
        records.push(vec![
            // This is required only to have min and max values.
            monitor_value_for_chunk_size
                .execution_timing
                .whole_duration
                .as_millis() as u32,
            monitor_value_for_chunk_size.chunk_size,
            monitor_value_for_chunk_size.param_1,
            monitor_value_for_chunk_size.param_2,
            monitor_value_for_chunk_size.param_3,
            monitor_value_for_chunk_size.param_4,
        ]);
    }

    let mut colors = vec![
        GREY,       // Duration // Actually never used, but it's important that stays here.
        PURPLE,     // Chunk Size
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

    let diagram_min_y_drawn_for_bar = 300;
    let diagram_max_y = 10000;
    let leeway_height_for_placing_ys = diagram_max_y - diagram_min_y_drawn_for_bar;

    let mut i_block_of_columns = 0;
    while i_block_of_columns < execution_generics_list.len() {
        let monitor_value_for_chunk_size = &execution_generics_list[i_block_of_columns];

        let execution_timing = &monitor_value_for_chunk_size.execution_timing;
        let ratios = vec![
            execution_timing.prop_p11,
            execution_timing.prop_p12,
            execution_timing.prop_p21,
            execution_timing.prop_p22,
            execution_timing.prop_p23,
            execution_timing.prop_p24,
            execution_timing.prop_p3,
            execution_timing.prop_extra,
        ];

        let record = &records[i_block_of_columns];
        let chunk_size = record[1];
        let mut group_of_bars = GroupOfBars::new();

        let mut curr_x = chunk_size * num_cols_per_data_item;

        // Composite Bar: durations spread in full height
        {
            let mut composite_bar = SingleBar::new();
            let mut curr_y_bottom = 0;

            let mut i_ratio = 0;
            for ratio in &ratios {
                let proportional_value = (ratio * (diagram_max_y as f64)) as i32;
                composite_bar.add_rectangle(
                    //
                    SingleBarRectangle::new(
                        curr_x,
                        curr_y_bottom,
                        curr_y_bottom + proportional_value,
                        colors_for_partial_durations[i_ratio],
                    ),
                );
                curr_y_bottom += proportional_value;

                i_ratio += 1;
            }

            group_of_bars.add_bar(composite_bar);
            curr_x += 1;
        }

        let mut i_column_in_this_block = 0;

        // Composite Bar: durations spread in actual height
        {
            let mut composite_bar = SingleBar::new();
            let mut curr_y_bottom = 0;

            let min_column = min_values[i_column_in_this_block];
            let max_column = max_values[i_column_in_this_block];
            let diff_max_min_column = max_column - min_column;

            let value = record[i_column_in_this_block];
            let percentage = if diff_max_min_column == 0 {
                // If all data are the same, we use a 50% as default value.
                0.5
            } else {
                (value - min_column) as f64 / diff_max_min_column as f64
            };
            let this_max_height = // Value "min_height" is included.
                diagram_min_y_drawn_for_bar + (percentage * (leeway_height_for_placing_ys as f64)) as i32;

            let mut i_ratio = 0;
            for ratio in &ratios {
                let proportional_value = (ratio * (this_max_height as f64)) as i32;
                let single_bar_rectangle = SingleBarRectangle::new(
                    curr_x,
                    curr_y_bottom,
                    curr_y_bottom + proportional_value,
                    colors_for_partial_durations[i_ratio],
                );
                composite_bar.add_rectangle(single_bar_rectangle);
                curr_y_bottom += proportional_value;
                i_ratio += 1;
            }

            group_of_bars.add_bar(composite_bar);
            curr_x += 1;
            i_column_in_this_block += 1;
        }

        // Single Bars
        while i_column_in_this_block < record.len() {
            let min_column = min_values[i_column_in_this_block];
            let max_column = max_values[i_column_in_this_block];
            let diff_max_min_column = max_column - min_column;

            let value = record[i_column_in_this_block];
            let percentage = if diff_max_min_column == 0 {
                // If all data are the same, we use a 50% as default value.
                0.5
            } else {
                (value - min_column) as f64 / diff_max_min_column as f64
            };
            let proportional_value = // Value "min_height" is included.
                diagram_min_y_drawn_for_bar + (percentage * (leeway_height_for_placing_ys as f64)) as i32;

            let mut single_bar = SingleBar::new();
            single_bar.add_rectangle(
                //
                SingleBarRectangle::new(
                    //
                    curr_x,
                    0,
                    proportional_value,
                    colors[i_column_in_this_block],
                ),
            );
            group_of_bars.add_bar(single_bar);

            curr_x += 1;
            i_column_in_this_block += 1;
        }
        groups_of_bars.push(group_of_bars);

        i_block_of_columns += 1;
    }

    let bar_plot = BarPlot::new(
        3600,
        1400,
        format!(
            "Diagram: {}, Chunk Size from {} to {}",
            fasta_file_name, min_chunk_size, max_chunk_size
        ),
    );
    bar_plot.draw(
        get_path_for_plot_file(fasta_file_name, min_chunk_size, max_chunk_size),
        num_cols_per_data_item,
        min_chunk_size as u32, // min_x,
        max_chunk_size as u32, // max_x,
        diagram_max_y,
        groups_of_bars,
    );
}
