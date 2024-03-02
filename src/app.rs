mod graphics;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct App {}

impl App {
    pub async fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let builder = WindowBuilder::new();
        let window = builder.build(&event_loop).unwrap();
        let mut app_window = graphics::AppWindow::init(&window).await;

        let window = &window;
        let app_window = &mut app_window;
        event_loop
            .run(move |event, target| {
                if let Event::WindowEvent {
                    window_id: _,
                    event,
                } = event
                {
                    match event {
                        WindowEvent::Resized(new_size) => app_window.resize(&window, new_size),
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::RedrawRequested => app_window.render(),
                        _ => {}
                    };
                }
            })
            .unwrap();
    }

    pub fn init() -> App {
        env_logger::init();

        App {}
    }
}
