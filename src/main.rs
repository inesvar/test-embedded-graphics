//! # Example: Primitive stroke styles
//!
//! This example demonstrates the different stroke styles available for primitives.

#[allow(unused_imports)]
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, CornerRadii, Ellipse, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle,
        RoundedRectangle, StrokeAlignment, StrokeStyle, Triangle,
    },
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

const PADDING: i32 = 16;
const RECTANGLE_SIZES: [Size; 3] = [Size::new(10, 64), Size::new(42, 42), Size::new(48, 24)];
const COLORS: [Rgb888; 3] = [
    Rgb888::new(0xF8, 0xF4, 0xE3),
    Rgb888::new(0x58, 0xA4, 0xB0),
    Rgb888::new(0x0C, 0x7C, 0x59),
];

/// Draws all embedded-graphics primitives.
fn draw_primitives<D>(target: &mut D, w: u32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    for i in 0..3 {
        let rectangle_top_left = Point::new(64, 64) - RECTANGLE_SIZES[i];
        let fill_style = PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(COLORS[i], w))
            .stroke_alignment(StrokeAlignment::Center)
            .build();
        let dot_style = PrimitiveStyleBuilder::from(&fill_style)
            .stroke_color(COLORS[(i + 1) % 3])
            .stroke_style(Some(StrokeStyle::Dotted))
            .build();

        Rectangle::new(rectangle_top_left, RECTANGLE_SIZES[i])
            .translate(Point::new(0, (64 + PADDING) * i as i32))
            .into_styled(fill_style)
            .draw(target)?;
        Rectangle::new(rectangle_top_left, RECTANGLE_SIZES[i])
            .translate(Point::new(0, (64 + PADDING) * i as i32))
            .into_styled(dot_style)
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
