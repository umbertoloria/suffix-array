use crate::suffix_array::prefix_trie::PrefixTrieMonitor;
use plotters::prelude::full_palette::GREY_800;
use plotters::prelude::*;
use plotters::style::full_palette::{BLUE_400, GREY, ORANGE_500};
use std::time::Duration;

pub fn draw_histogram_from_prefix_trie_monitor(
    data_list: Vec<(usize, Duration, PrefixTrieMonitor)>,
) {
    let num_cols_per_data_item: u32 = 1 + 4; // Duration + 4 monitor parameters.
    let chunk_size = (
        // Min
        data_list.first().unwrap().0 as u32 * num_cols_per_data_item,
        // Max
        data_list.last().unwrap().0 as u32 * num_cols_per_data_item,
    );

    let root_area = BitMapBackend::new("./plots/plot.png", (3600, 1400)).into_drawing_area();
    // root_area.fill(&WHITE).unwrap();
    root_area.fill(&GREY_800).unwrap();

    // let max_height = 2500;
    let max_height = 10000;
    let plot_title = "Prefix Trie: Monitor Data";
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(plot_title, ("sans-serif", 40))
        .build_cartesian_2d(
            (chunk_size.0..chunk_size.1 + 10).into_segmented(),
            0..max_height,
        )
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let sets_or_bars = data_list.into_iter().map(|item| {
        let (chunk_size, duration, monitor) = item;
        let chunk_size = chunk_size as u32;

        let bar_millis = create_rectangle_bar(
            chunk_size * num_cols_per_data_item,
            (duration.as_millis() * 3) as i32,
            GREY,
        );
        let bar_cmp_rules = create_rectangle_bar(
            num_cols_per_data_item * chunk_size + 1,
            (monitor.compares_using_rules * 300) as i32,
            GREEN,
        );
        let bar_cmp_str = create_rectangle_bar(
            num_cols_per_data_item * chunk_size + 2,
            (monitor.compares_using_strcmp / 50) as i32,
            RED,
        );
        let bar_cmp_one_cf = create_rectangle_bar(
            num_cols_per_data_item * chunk_size + 3,
            (monitor.compares_with_one_cf * 5) as i32,
            BLUE_400,
        );
        let bar_cmp_two_cf = create_rectangle_bar(
            num_cols_per_data_item * chunk_size + 4,
            (monitor.compares_with_two_cfs / 22) as i32,
            ORANGE_500,
        );

        (
            bar_millis,
            bar_cmp_rules,
            bar_cmp_str,
            bar_cmp_one_cf,
            bar_cmp_two_cf,
        )
    });
    let mut flat_bars = Vec::new();
    for (a, b, c, d, e) in sets_or_bars {
        flat_bars.push(a);
        flat_bars.push(b);
        flat_bars.push(c);
        flat_bars.push(d);
        flat_bars.push(e);
    }
    ctx.draw_series(flat_bars).unwrap();
}

fn create_rectangle_bar(x: u32, y: i32, color: RGBColor) -> Rectangle<(SegmentValue<u32>, i32)> {
    let mut bar = Rectangle::new(
        [
            //
            (SegmentValue::Exact(x), 0),
            (SegmentValue::Exact(x + 1), y),
        ],
        color.filled(),
    );
    bar.set_margin(0, 0, 2, 2);
    bar
}
