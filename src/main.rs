mod app;
// mod dns;
// mod http;

fn main() {
    let mut application = app::App::init();

    futures::executor::block_on(application.run());
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() != 2 {
    //     panic!("Usage: {} <address>", args[0]);
    // }
    // let address = &args[1];

    // println!("Opening: {}", address);

    // let _content = http::request_content(address).unwrap();
    // println!("Content: {}", content.body.unwrap());
}
