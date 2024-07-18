use json::parse;

mod conditionals;
mod basics;
mod io_json;
mod ownership;
mod parse_json;

fn main() {
    // basics::run();
    // conditionals::run();
    // ownership::run();

    // io_json::run();


    let _ = parse_json::run();

}

