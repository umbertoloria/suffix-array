use crate::suffix_array::prefix_trie::PrefixTrieMonitor;
use plotters::prelude::full_palette::GREY_800;
use plotters::prelude::*;
use plotters::style::full_palette::{BLUE_400, GREY, ORANGE_500};
use std::time::Duration;

pub struct GroupOfBars {
    pub bars: Vec<SingleBar>,
}
impl GroupOfBars {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }
    pub fn add_bar(&mut self, bar: SingleBar) {
        self.bars.push(bar);
    }
    pub fn get_bars_count(&self) -> usize {
        self.bars.len()
    }
    pub fn get_bar(&self, index: usize) -> &SingleBar {
        &self.bars[index]
    }
}
pub struct SingleBar {
    pub x: u32,
    pub y: i32,
    pub color: RGBColor,
}

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

    let groups_of_bars = data_list.into_iter().map(|item| {
        let (chunk_size, duration, monitor) = item;
        let chunk_size = chunk_size as u32;
        let mut result = GroupOfBars::new();
        result.add_bar(SingleBar {
            x: chunk_size * num_cols_per_data_item,
            y: (duration.as_millis() * 3) as i32,
            color: GREY,
        });
        result.add_bar(SingleBar {
            x: num_cols_per_data_item * chunk_size + 1,
            y: (monitor.compares_using_rules * 300) as i32,
            color: GREEN,
        });
        result.add_bar(SingleBar {
            x: num_cols_per_data_item * chunk_size + 2,
            y: (monitor.compares_using_strcmp / 50) as i32,
            color: RED,
        });
        result.add_bar(SingleBar {
            x: num_cols_per_data_item * chunk_size + 3,
            y: (monitor.compares_with_one_cf * 5) as i32,
            color: BLUE_400,
        });
        result.add_bar(SingleBar {
            x: num_cols_per_data_item * chunk_size + 4,
            y: (monitor.compares_with_two_cfs / 22) as i32,
            color: ORANGE_500,
        });
        result
    });
    let mut flat_bars = Vec::new();
    for group_of_bars in groups_of_bars {
        for i in 0..group_of_bars.get_bars_count() {
            let bar = group_of_bars.get_bar(i);
            flat_bars.push(create_rectangle_bar(bar.x, bar.y, bar.color));
        }
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
