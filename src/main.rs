mod render;
mod renderloop;
mod sim;
mod timestep;

use crate::renderloop::{RenderLoop, RenderLoopCreateError};
use femtovg::{Color, LineCap, LineJoin, Paint, Path};

#[derive(Debug, thiserror::Error)]
pub enum TrafficScapeError {
    #[error("failed to initialize trafficscape renderloop")]
    RenderLoopCreation(#[from] RenderLoopCreateError),
}

fn main() -> Result<(), TrafficScapeError> {
    // Aim for 60 updates per second
    let traffic_scape = RenderLoop::new(1.0 / 60.0)?;

    // Start the simulation loop
    traffic_scape.run_loop(
        |_| {},
        |_| {},
        |_, scale, canvas| {
            // To make life much, much easier
            let scale = scale as f32;

            // Drawing style
            let mut paint = Paint::color(Color::rgbf(1.0, 1.0, 1.0));
            paint.set_line_cap(LineCap::Square);
            paint.set_line_join(LineJoin::Miter);
            paint.set_line_width(scale * 4.0);
            let paint = paint;

            // Render line path
            let mut line_path = Path::new();
            line_path.move_to(scale * 100.0, scale * 100.0);
            line_path.line_to(scale * 200.0, scale * 115.0);
            line_path.line_to(scale * 300.0, scale * 400.0);
            canvas.stroke_path(&mut line_path, paint);
        },
    );

    println!("exiting");

    Ok(())
}
