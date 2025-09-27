use internet_reloader::app;
fn main() {
    let app = app::NetworkApp::default();

    app.poll();
}
