use crate::plot::vendor::draw_histogram;
use crate::suffix_array::prefix_trie::PrefixTrieMonitor;
use plotters::prelude::{RGBColor, GREEN, RED};
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
    let min_x = data_list.first().unwrap().0 as u32;
    let max_x = data_list.last().unwrap().0 as u32;
    let groups_of_bars = data_list
        .into_iter()
        .map(|item| {
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
        })
        .collect::<Vec<GroupOfBars>>();
    draw_histogram(
        "./plots/plot.png",
        3600,
        1400,
        "Prefix Trie: Monitor Data",
        num_cols_per_data_item,
        min_x,
        max_x,
        groups_of_bars,
    );
}
