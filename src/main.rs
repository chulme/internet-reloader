use internet_reloader::app;
fn main() {
    let app = app::NetworkApp::default();
    let res = app.poll();

    println!("{}", res);
}
