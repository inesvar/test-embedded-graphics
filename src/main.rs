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
const COLORS: [Rgb888; 3] = [
    Rgb888::new(0xF8, 0xF4, 0xE3),
    Rgb888::new(0x58, 0xA4, 0xB0),
    Rgb888::new(0x0C, 0x7C, 0x59),
];

/// Draws all embedded-graphics primitives.
fn draw_primitives<D>(target: &mut D, rectangle_size: Size) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    for i in 0..2 {
        let rectangle_top_left = Point::new(16, 16) - rectangle_size;
        let fill_style =
            PrimitiveStyleBuilder::from(&PrimitiveStyle::with_stroke(COLORS[i % 3], 1 + i as u32))
                .build();
        let dot_style = PrimitiveStyleBuilder::from(&fill_style)
            .stroke_color(COLORS[(i + 1) % 3])
            .stroke_style(Some(StrokeStyle::Dotted))
            .build();

        let rectangle = Rectangle::new(rectangle_top_left, rectangle_size)
            .translate(Point::new(0, (16 + PADDING) * i as i32));
        rectangle.into_styled(fill_style).draw(target)?;
        rectangle.into_styled(dot_style).draw(target)?;
    }

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(256, 64));

    // Draw the primitives using a thin stroke.
    //
    // Instead of directly drawing to the display a `TranslatedDrawTarget` is created by
    // using `display.translated(position)`. This translates all drawing operations in `draw_shapes`
    // by 10 pixels in the x and y direction.
    let mut position = Point::new(8, 8);

    for size in 12..=24 {
        draw_primitives(&mut display.translated(position), Size::new_equal(size))?;

        position.x += 16 + PADDING;
    }

    Window::new("Strokes", &OutputSettings::default()).show_static(&display);

    Ok(())
}
