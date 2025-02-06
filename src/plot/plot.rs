use crate::plot::interface::{BarPlot, GroupOfBars, SingleBar, SingleBarRectangle};
use crate::suffix_array::monitor::Monitor;
use plotters::prelude::full_palette::{BLUE_400, GREY, ORANGE_500};
use plotters::prelude::{GREEN, RED};
use plotters::style::full_palette::PURPLE;

pub fn draw_plot_from_monitor(
    fasta_file_name: &str,
    chunk_and_monitor_pairs: Vec<(usize, Monitor)>,
) {
    let num_cols_per_data_item: u32 = 1 + 1 + 4; // Duration + 4 monitor parameters.
    let min_x = chunk_and_monitor_pairs
        .first()
        .expect("Data List should no be empty")
        .0 as u32;
    let max_x = chunk_and_monitor_pairs.last().unwrap().0 as u32;
    let mut groups_of_bars = Vec::new();

    let records = chunk_and_monitor_pairs
        .into_iter()
        .map(|(chunk_size, monitor)| {
            let chunk_size = chunk_size as u32;
            let duration = monitor.get_process_duration().unwrap();
            let value_1 = duration.as_millis() as u32;
            let value_2 = monitor.compares_using_rules as u32;
            let value_3 = monitor.compares_using_strcmp as u32;
            let value_4 = monitor.compares_with_one_cf as u32;
            let value_5 = monitor.compares_with_two_cfs as u32;
            vec![chunk_size, value_1, value_2, value_3, value_4, value_5]
        })
        .collect::<Vec<_>>();

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
    for record in records {
        let chunk_size = record[0];
        let mut group_of_bars = GroupOfBars::new();
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
            let proportional_value =
                min_height + (percentage * (leeway_height_for_displaying_values as f64)) as i32;

            /*println!(
                "{j}: VALUES WAS {} / min={} MAX={}, perc={}, proportional={}",
                value, min_column, max_column, percentage, proportional_value
            );*/

            let mut single_bar = SingleBar::new();
            single_bar.add_rectangle(
                //
                SingleBarRectangle::new(
                    chunk_size * num_cols_per_data_item + (j as u32),
                    0,
                    proportional_value,
                    colors[j],
                ),
            );
            group_of_bars.add_bar(single_bar);
        }
        groups_of_bars.push(group_of_bars);
    }

    /*
    for i in 0..groups_of_bars.len() {
        let group_of_bars = &groups_of_bars[i];
        println!(" group of bars index {i}");
        for j in 0..group_of_bars.get_bars_count() {
            let bar = group_of_bars.get_bar(j);
            println!("{j}: {:?}", bar);
        }
    }
    */

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
