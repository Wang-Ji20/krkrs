//! cli interface
use krkrs::interface::cli::App;

fn main() {
    let mut app = App::new_cli_from_ks("public/lorerei.ks");
    app.run();
}
