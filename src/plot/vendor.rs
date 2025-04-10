use crate::plot::interface::GroupOfBars;
use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::element::Rectangle;
use plotters::prelude::full_palette::GREY_800;
use plotters::prelude::{Color, IntoDrawingArea, IntoSegmentedCoord, RGBColor, SegmentValue};

pub fn draw_plot(
    path: &str,
    width: u32,
    height: u32,
    plot_title: &str,
    num_cols_per_data_item: u32,
    min_x: u32,
    max_x: u32,
    max_height: i32,
    groups_of_bars: &Vec<GroupOfBars>,
) {
    let root_area = BitMapBackend::new(path, (width, height)).into_drawing_area();
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
        .caption(plot_title, ("sans-serif", 40))
        .build_cartesian_2d((x_range.0..x_range.1 + 10).into_segmented(), 0..max_height)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    let mut flat_bars = Vec::new();
    for group_of_bars in groups_of_bars {
        for i in 0..group_of_bars.get_bars_count() {
            let bar = group_of_bars.get_bar(i);
            let rectangle_bars = bar.create_rectangle();
            for rectangle_bar in rectangle_bars {
                flat_bars.push(rectangle_bar);
            }
        }
    }
    ctx.draw_series(flat_bars).unwrap();
}

pub fn create_rectangle_bar(
    x: u32,
    min_y: i32,
    max_y: i32,
    color: RGBColor,
) -> Rectangle<(SegmentValue<u32>, i32)> {
    let mut bar = Rectangle::new(
        [
            //
            (SegmentValue::Exact(x), min_y),
            (SegmentValue::Exact(x + 1), max_y),
        ],
        color.filled(),
    );
    bar.set_margin(0, 0, 2, 2);
    bar
}
