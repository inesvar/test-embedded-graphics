//! # Example: Primitive stroke styles
//!
//! This example demonstrates the different stroke styles available for primitives.

#[allow(unused_imports)]
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, CornerRadii, Ellipse, Line, PrimitiveStyle, Rectangle, RoundedRectangle, Triangle,
    },
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

const PADDING: i32 = 16;

/// Draws all embedded-graphics primitives.
fn draw_primitives<D>(target: &mut D, w: u32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    let rectangle_size = Size::new(10, 64);
    let rectangle_top_left = Point::new(64, 64) - rectangle_size;
    let colors = [
        Rgb888::CSS_ORANGE_RED,
        Rgb888::CSS_CRIMSON,
        Rgb888::CSS_AQUAMARINE,
    ];

    for i in 0..3 {
        Rectangle::new(rectangle_top_left, rectangle_size)
            .translate(Point::new((64 + PADDING) * i, 0))
            .into_styled(PrimitiveStyle::with_stroke(
                colors[i as usize],
                w + i as u32,
            ))
            .draw(target)?;
    }

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(512, 256));

    // Draw the primitives using a thin stroke.
    //
    // Instead of directly drawing to the display a `TranslatedDrawTarget` is created by
    // using `display.translated(position)`. This translates all drawing operations in `draw_shapes`
    // by 10 pixels in the x and y direction.
    let mut position = Point::new(10, 10);
    draw_primitives(&mut display.translated(position), 4)?;

    // Draw the primitives using a medium stroke.
    position.y += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 10)?;

    // Draw the primitives using a thick stroke.
    position.y += 64 + PADDING;
    draw_primitives(&mut display.translated(position), 25)?;

    Window::new("Strokes", &OutputSettings::default()).show_static(&display);

    Ok(())
}
