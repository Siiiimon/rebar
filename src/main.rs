mod app;
// mod dns;
// mod http;
use winit::event_loop::EventLoop;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new();
    let window = builder.build(&event_loop).unwrap();

    futures::executor::block_on(app::App::run(event_loop, window));
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() != 2 {
    //     panic!("Usage: {} <address>", args[0]);
    // }
    // let address = &args[1];

    // println!("Opening: {}", address);

    // let _content = http::request_content(address).unwrap();
    // println!("Content: {}", content.body.unwrap());
}
