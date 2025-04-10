use crate::plot::vendor::{create_rectangle_bar, draw_plot};
use plotters::element::Rectangle;
use plotters::prelude::{RGBColor, SegmentValue};

#[derive(Debug)]
pub struct CompositeBarRectangle {
    pub x: u32,
    pub y_bottom: i32,
    pub y_top: i32,
    pub color: RGBColor,
}
impl CompositeBarRectangle {
    pub fn new(x: u32, y_bottom: i32, y_top: i32, color: RGBColor) -> Self {
        Self {
            x,
            y_bottom,
            y_top,
            color,
        }
    }
    pub fn create_rectangle(&self) -> Rectangle<(SegmentValue<u32>, i32)> {
        create_rectangle_bar(self.x, self.y_bottom, self.y_top, self.color)
    }
}

#[derive(Debug)]
pub struct CompositeBar {
    rectangles: Vec<CompositeBarRectangle>,
}
impl CompositeBar {
    pub fn new() -> Self {
        Self {
            rectangles: Vec::new(),
        }
    }
    pub fn new_only_one(single_rectangle: CompositeBarRectangle) -> Self {
        Self {
            rectangles: vec![single_rectangle],
        }
    }
    pub fn add_rectangle(&mut self, rectangle: CompositeBarRectangle) {
        self.rectangles.push(rectangle);
    }
    pub fn create_rectangle(&self) -> Vec<Rectangle<(SegmentValue<u32>, i32)>> {
        let mut result = Vec::new();
        for rectangle in &self.rectangles {
            let main_rectangle = rectangle.create_rectangle();
            result.push(main_rectangle);
        }
        result
    }
}

pub struct GroupOfBars {
    pub bars: Vec<CompositeBar>,
}
impl GroupOfBars {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }
    pub fn new_only_one(single_composite_bar: CompositeBar) -> Self {
        Self {
            bars: vec![single_composite_bar],
        }
    }
    pub fn add_bar(&mut self, bar: CompositeBar) {
        self.bars.push(bar);
    }
    pub fn get_bars_count(&self) -> usize {
        self.bars.len()
    }
    pub fn get_bar(&self, index: usize) -> &CompositeBar {
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
        path: &str,
        num_cols_per_data_item: u32,
        min_x: u32,
        max_x: u32,
        max_height: i32,
        groups_of_bars: &Vec<GroupOfBars>,
    ) {
        draw_plot(
            path,
            self.width,
            self.height,
            &self.plot_title,
            num_cols_per_data_item,
            min_x,
            max_x,
            max_height,
            groups_of_bars,
        );
    }
}
