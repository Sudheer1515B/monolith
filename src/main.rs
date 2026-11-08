mod scan;
fn main() {
    for app in scan::scanApps() {
        println!("{} {}", app._name, app.version.unwrap_or_default())
    }
}
