use crate::plot::vendor::{create_rectangle_bar, draw_plot};
use plotters::element::Rectangle;
use plotters::prelude::{RGBColor, SegmentValue};

#[derive(Debug)]
pub struct SingleBar {
    pub x: u32,
    pub y: i32,
    pub color: RGBColor,
}
impl SingleBar {
    pub fn new(x: u32, y: i32, color: RGBColor) -> Self {
        Self { x, y, color }
    }
    pub fn create_rectangle(
        &self,
        margin_right: Option<u32>,
    ) -> Vec<Rectangle<(SegmentValue<u32>, i32)>> {
        let mut result = Vec::new();

        let main_rectangle = create_rectangle_bar(self.x, 0, self.y, self.color, margin_right);
        result.push(main_rectangle);

        result
    }
}

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

pub struct BarPlot {
    pub width: u32,
    pub height: u32,
    pub plot_title: String,
}
impl BarPlot {
    pub fn new(width: u32, height: u32, plot_title: String) -> Self {
        Self {
            width,
            height,
            plot_title,
        }
    }
    pub fn draw(
        &self,
        path: String,
        num_cols_per_data_item: u32,
        min_x: u32,
        max_x: u32,
        max_height: i32,
        groups_of_bars: Vec<GroupOfBars>,
    ) {
        draw_plot(
            path,
            self.width,
            self.height,
            // TODO: Is this cloning?
            self.plot_title.to_string(),
            num_cols_per_data_item,
            min_x,
            max_x,
            max_height,
            groups_of_bars,
        );
    }
}
