// inspired by: https://github.com/embedded-graphics/examples

#![no_std]
extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

#[allow(unused)]
use core::cmp::max;
use embedded_graphics::{
    geometry::{real::real_impl::Real, Point, Size},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

const PADDING: i32 = 16;
const SIZES: [Size; 3] = [Size::new(10, 64), Size::new(42, 42), Size::new(48, 24)];
const COLORS: [Rgb888; 3] = [
    Rgb888::new(0xF8, 0xF4, 0xE3),
    Rgb888::new(0x58, 0xA4, 0xB0),
    Rgb888::new(0x0C, 0x7C, 0x59),
];

fn dot_positions_with_dotted_corners(size: Size, dot_size: u32) -> impl Iterator<Item = Size> {
    let length = (size.width * size.width + size.height * size.height).isqrt();
    let nb_dots = length / (2 * dot_size);
    // let nb_dots_x = size.width / (2 * dot_size);
    // let nb_dots_y = size.height / (2 * dot_size);
    // let nb_dots = max(nb_dots_x, nb_dots_y);
    let dot_offset_x = Real::from(size.width) / Real::from(nb_dots);
    let dot_offset_y = Real::from(size.height) / Real::from(nb_dots);

    let idx_iter = 0..=nb_dots;
    idx_iter.map(move |idx| {
        Size::new(
            (dot_offset_x * Real::from(idx)).round().into(),
            (dot_offset_y * Real::from(idx)).round().into(),
        )
    })
}

/// Draws all embedded-graphics primitives.
fn draw_primitives<D>(target: &mut D, w: u32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    for i in 0..3 {
        let mut rectangle_top_left = Point::new(64, 64) - SIZES[i];
        let line_style = PrimitiveStyle::with_stroke(COLORS[i], w);
        let fill_style = PrimitiveStyle::with_fill(COLORS[(i + 1) % 3]);

        /* Rectangle::with_center(rectangle_top_left, Size::new_equal(w))
        .translate(Point::new(0, (64 + PADDING) * i as i32))
        .into_styled(fill_style)
        .draw(target)?; */

        Line::new(rectangle_top_left, rectangle_top_left + SIZES[i])
            .translate(Point::new(0, (64 + PADDING) * i as i32))
            .into_styled(line_style)
            .draw(target)?;

        rectangle_top_left += Size::new(4 * w, 0);

        Line::new(rectangle_top_left, rectangle_top_left + SIZES[i])
            .translate(Point::new(0, (64 + PADDING) * i as i32))
            .into_styled(line_style)
            .draw(target)?;

        for translation in dot_positions_with_dotted_corners(SIZES[i], w) {
            Circle::with_center(rectangle_top_left + translation, w)
                .translate(Point::new(0, (64 + PADDING) * i as i32))
                .into_styled(fill_style)
                .draw(target)?;
        }

        rectangle_top_left += Size::new(4 * w, 0);

        for translation in dot_positions_with_dotted_corners(SIZES[i], w) {
            Circle::with_center(rectangle_top_left + translation, w)
                .translate(Point::new(0, (64 + PADDING) * i as i32))
                .into_styled(fill_style)
                .draw(target)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(512, 256));

    let mut position = Point::new(10, 16);
    draw_primitives(&mut display.translated(position), 1)?;

    position.x += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 2)?;

    position.x += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 3)?;

    position.x += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 4)?;

    position.x += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 8)?;

    position.x += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 16)?;

    Window::new("Strokes", &OutputSettings::default()).show_static(&display);

    Ok(())
}
