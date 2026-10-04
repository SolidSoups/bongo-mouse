use std::sync::Arc;
use std::time::Instant;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
    platform::windows::WindowAttributesExtWindows,
};

use crate::gpu::{Frame, Gpu};

pub trait Game {
    fn window_attributes() -> WindowAttributes {
        Window::default_attributes()
    }

    fn new(gpu: &Gpu) -> Self;

    fn update(&mut self, _dt: f32) {}

    fn render(&mut self, gpu: &Gpu, frame: &mut Frame);

    fn window_event(&mut self, _window: &Window, _event: &WindowEvent) {}
}

struct State<G: Game> {
    window: Arc<Window>,
    gpu: Gpu,
    game: G,
}

impl<G: Game> State<G> {
    fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        let Some(mut frame) = self.gpu.begin_frame()? else {
            return Ok(());
        };
        self.game.render(&self.gpu, &mut frame);
        self.gpu.end_frame(frame);

        Ok(())
    }
}

struct App<G: Game> {
    state: Option<State<G>>,
    last_time: Instant,
}

impl<G: Game> ApplicationHandler for App<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let mut attributes = G::window_attributes();
        let transparent = attributes.transparent;
        if transparent {
            attributes = attributes.with_no_redirection_bitmap(true);
        }
        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        let gpu = pollster::block_on(Gpu::new(window.clone(), transparent)).unwrap();
        let game = G::new(&gpu);

        self.state = Some(State { window, gpu, game } );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent
    ) {
        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };

        state.game.window_event(&state.window, &event);
        
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(size) => state.gpu.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                let dt = self.last_time.elapsed();
                self.last_time = Instant::now();
                state.game.update(dt.as_secs_f32());
                if let Err(e) = state.render() {
                    log::error!("{e}");
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}

pub fn run<G: Game>() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    let mut app = App::<G> {
        state: None,
        last_time: Instant::now(),
    };
    event_loop.run_app(&mut app)?;

    Ok(())
}
