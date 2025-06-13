use crate::files::paths::get_path_for_plot_file;
use crate::plot::interface::{BarPlot, CompositeBar, CompositeBarRectangle, GroupOfBars};
use plotters::prelude::full_palette::{GREEN_500, GREY_500, ORANGE_300, PURPLE_500};
use plotters::style::RGBColor;

pub fn draw_plot_from_monitor(
    fasta_file_name: &str,
    classic_computation_duration_micros: u64,
    chunk_size_and_phase_micros_list: Vec<(usize, (u64, u64, u64))>,
    max_duration_in_micros: u32,
) {
    let diagram_max_y = 10000;
    let abs_max_value = max_duration_in_micros as i32;

    let mut curr_x = 1;
    let mut groups_of_bars = Vec::new();

    // Innovative Technique Executions
    for (_, micros) in &chunk_size_and_phase_micros_list {
        groups_of_bars.push(
            // Composite Vertical Bar
            GroupOfBars::new_only_one(
                //
                create_composite_bar_from_parts(
                    curr_x,
                    vec![
                        (micros.0 as i32, GREY_500),   // Factorization phase
                        (micros.1 as i32, ORANGE_300), // Tree phase
                        (micros.2 as i32, GREEN_500),  // Suffix Array phase
                    ],
                    abs_max_value,
                    diagram_max_y,
                ),
            ),
        );
        curr_x += 1;
    }

    // Classic Technique Execution
    groups_of_bars.push(
        //
        GroupOfBars::new_only_one(
            //
            CompositeBar::new_only_one(
                //
                CompositeBarRectangle::new(
                    //
                    curr_x,
                    0,
                    proportional_value(
                        classic_computation_duration_micros as i32,
                        abs_max_value,
                        diagram_max_y,
                    ),
                    PURPLE_500,
                ),
            ),
        ),
    );
    curr_x += 1;

    let min_chunk_size = chunk_size_and_phase_micros_list.first().unwrap().0;
    let max_chunk_size = chunk_size_and_phase_micros_list.last().unwrap().0;
    let bar_plot = BarPlot::new(
        3600,
        1200,
        format!(
            "Diagram: {}, Chunk Size from {} to {}",
            fasta_file_name, min_chunk_size, max_chunk_size
        ),
    );
    bar_plot.draw(
        &get_path_for_plot_file(fasta_file_name, min_chunk_size, max_chunk_size),
        1,
        1, // min_x,
        curr_x,
        diagram_max_y,
        &groups_of_bars,
    );
}

fn proportional_value(absolute_value: i32, abs_max_value: i32, relative_spacing: i32) -> i32 {
    let result = absolute_value as f32 / abs_max_value as f32 * (relative_spacing as f32);
    result as i32
}

fn create_composite_bar_from_parts(
    x: u32,
    parts: Vec<(i32, RGBColor)>,
    abs_max_value: i32,
    diagram_max_y: i32,
) -> CompositeBar {
    let mut composite_bar = CompositeBar::new();
    let mut curr_y = 0;
    for (duration, color) in parts {
        composite_bar.add_rectangle(
            //
            CompositeBarRectangle::new(
                //
                x,
                proportional_value(curr_y, abs_max_value, diagram_max_y),
                proportional_value(curr_y + duration, abs_max_value, diagram_max_y),
                color,
            ),
        );
        curr_y += duration;
    }
    composite_bar
}
