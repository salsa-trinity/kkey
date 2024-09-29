use kkey::app::App;

fn main() {
    let mut app = App::new();
    while app.is_open {
        app.draw();
    }
}
