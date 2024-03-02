mod app;
mod dns;
mod http;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <address>", args[0]);
    }
    let address = &args[1];
    println!("Opening: {}", address);

    let mut application = app::App::init();
    let app_future = application.run();

    let content = http::request_content(address).unwrap();
    println!("Content: {}", content.body.unwrap());

    futures::executor::block_on(app_future);
}
