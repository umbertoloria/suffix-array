use crate::plot::plot::GroupOfBars;
use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::element::Rectangle;
use plotters::prelude::full_palette::GREY_800;
use plotters::prelude::{Color, IntoDrawingArea, IntoSegmentedCoord, RGBColor, SegmentValue};

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

pub fn draw_plot(
    path: String,
    width: u32,
    height: u32,
    plot_title: String,
    num_cols_per_data_item: u32,
    min_x: u32,
    max_x: u32,
    max_height: i32,
    groups_of_bars: Vec<GroupOfBars>,
) {
    let root_area = BitMapBackend::new(&path, (width, height)).into_drawing_area();
    // root_area.fill(&WHITE).unwrap();
    root_area.fill(&GREY_800).unwrap();

    let x_range = (
        // Min
        min_x * num_cols_per_data_item,
        // Max
        max_x * num_cols_per_data_item,
    );
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(&plot_title, ("sans-serif", 40))
        .build_cartesian_2d((x_range.0..x_range.1 + 10).into_segmented(), 0..max_height)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    let mut flat_bars = Vec::new();
    for group_of_bars in groups_of_bars {
        for i in 0..group_of_bars.get_bars_count() {
            let bar = group_of_bars.get_bar(i);
            flat_bars.push(create_rectangle_bar(
                bar.x,
                bar.y,
                bar.color,
                if i == group_of_bars.get_bars_count() - 1 {
                    Some(5)
                } else {
                    None
                },
            ));
        }
    }
    ctx.draw_series(flat_bars).unwrap();
}

fn create_rectangle_bar(
    x: u32,
    y: i32,
    color: RGBColor,
    margin_right: Option<u32>,
) -> Rectangle<(SegmentValue<u32>, i32)> {
    let mut bar = Rectangle::new(
        [
            //
            (SegmentValue::Exact(x), 0),
            (SegmentValue::Exact(x + 1), y),
        ],
        color.filled(),
    );
    if let Some(margin_right) = margin_right {
        bar.set_margin(0, 0, 2, margin_right);
    } else {
        bar.set_margin(0, 0, 2, 2);
    }
    bar
}
