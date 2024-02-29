mod graphics;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }

    pub fn run(&self) {
        graphics::run();
        println!("Hello, world!");
    }
}
