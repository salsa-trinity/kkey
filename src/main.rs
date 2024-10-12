use kkey::app::App;

fn main() {
    let mut app = App::new();
    app.init();
    while app.is_open {
        app.draw();
    }
}
