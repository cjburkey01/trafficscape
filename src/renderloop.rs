use crate::timestep::TimeStep;
use femtovg::renderer::OpenGl;
use femtovg::{Canvas, Color, ErrorKind};
use glutin::dpi::PhysicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::platform::run_return::EventLoopExtRunReturn;
use glutin::window::{Window, WindowBuilder};
use glutin::{
    ContextBuilder, ContextError, ContextWrapper, CreationError, PossiblyCurrent, WindowedContext,
};

#[derive(Debug, thiserror::Error)]
pub enum RenderLoopCreateError {
    #[error("failed to create glutin context")]
    GlutinCreationError(#[from] CreationError),

    #[error("opengl context error")]
    ContextError(#[from] ContextError),

    #[error("canvas rendering error")]
    CanvasError(#[from] ErrorKind),
}

pub struct RenderLoop {
    target_delta_time: f64,
    event_loop: EventLoop<()>,
    windowed_context: ContextWrapper<PossiblyCurrent, Window>,
    canvas: Canvas<OpenGl>,
}

impl RenderLoop {
    pub fn new(target_delta_time: f64) -> Result<Self, RenderLoopCreateError> {
        let event_loop = EventLoop::new();
        let windowed_context = Self::create_windowed_context("Please wait...", &event_loop)?;
        let canvas = Self::create_canvas(&windowed_context)?;

        Ok(Self {
            target_delta_time,
            event_loop,
            windowed_context,
            canvas,
        })
    }

    pub fn run_loop<
        EventFn: FnMut(Event<()>),
        UpdateFn: FnMut(&mut ControlFlow),
        RenderFn: FnMut(f64, f64, &mut Canvas<OpenGl>),
    >(
        mut self,
        mut on_event: EventFn,   // (Event) -> ()
        mut on_update: UpdateFn, // (ControlFlowRef) -> ()
        mut on_render: RenderFn, // (PartialTicks, CanvasRef) -> ()
    ) -> Self {
        let (event_loop, windowed_context, canvas, secs_per_update) = (
            &mut self.event_loop,
            &mut self.windowed_context,
            &mut self.canvas,
            self.target_delta_time,
        );

        // Keep track of loop timing
        let mut timestep = TimeStep::new();
        let mut lag = 0.0;

        // Start the event loop and return control when it terminates
        event_loop.run_return(|event, _, control_flow| {
            Self::event_handler(
                (&mut on_event, &mut on_update, &mut on_render),
                event,
                control_flow,
                (windowed_context, canvas, secs_per_update),
                (&mut timestep, &mut lag),
            )
        });

        // Return back to caller
        self
    }

    fn create_windowed_context(
        start_title: &str,
        event_loop: &EventLoop<()>,
    ) -> Result<ContextWrapper<PossiblyCurrent, Window>, RenderLoopCreateError> {
        // Set up the window
        let wb = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(640, 400))
            .with_resizable(true)
            .with_title(start_title);

        // Create the context within the window
        let windowed_context = ContextBuilder::new()
            .with_vsync(false)
            .build_windowed(wb, event_loop)?;

        // Make the context current and return it
        Ok(unsafe { windowed_context.make_current() }.map_err(|(_, e)| e)?)
    }

    fn create_canvas(
        windowed_context: &WindowedContext<PossiblyCurrent>,
    ) -> Result<Canvas<OpenGl>, ErrorKind> {
        // Create canvas from OpenGL bindings
        Canvas::new(OpenGl::new(|s| {
            windowed_context.get_proc_address(s) as *const _
        })?)
    }

    fn update_title(window: &Window, fps: u32) {
        window.set_title(&format!(
            "{} v{} | FPS: {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            fps,
        ));
    }

    fn event_handler<
        EventFn: FnMut(Event<()>),
        UpdateFn: FnMut(&mut ControlFlow),
        RenderFn: FnMut(f64, f64, &mut Canvas<OpenGl>),
    >(
        (on_event, on_update, on_render): (&mut EventFn, &mut UpdateFn, &mut RenderFn),
        event: Event<()>,
        control_flow: &mut ControlFlow,
        (windowed_context, canvas, secs_per_update): (
            &mut ContextWrapper<PossiblyCurrent, Window>,
            &mut Canvas<OpenGl>,
            f64,
        ),
        (timestep, lag): (&mut TimeStep, &mut f64),
    ) {
        // Reset control flow
        *control_flow = ControlFlow::Poll;

        // Check the event before passing it on
        match &event {
            // Exit the loop if the window should close
            Event::LoopDestroyed
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            // Update the canvas size when the window size changes
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => windowed_context.resize(*new_size),

            // Render a frame and update the simulation as many times as necessary
            Event::RedrawRequested(_) => {
                // Increment game loop timer and update FPS counter
                *lag += timestep.delta();

                // Update title with FPS
                if let Some(fps) = timestep.frame_rate() {
                    Self::update_title(windowed_context.window(), fps);
                }

                // Update the game until it's caught up
                while *lag >= secs_per_update {
                    // Update the game
                    on_update(control_flow);
                    *lag -= secs_per_update;
                }

                // Update canvas size
                let dpi_factor = windowed_context.window().scale_factor();
                let size = windowed_context.window().inner_size();
                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(
                    0,
                    0,
                    size.width as u32,
                    size.height as u32,
                    Color::rgbf(0.15, 0.15, 0.12),
                );

                // Call render code
                on_render(*lag, dpi_factor, canvas);

                // Push rendered frame
                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }

            // Request another frame once this one renders
            Event::MainEventsCleared => windowed_context.window().request_redraw(),

            // Ignore other types
            _ => {}
        }

        // Pass event to user
        on_event(event);
    }
}
