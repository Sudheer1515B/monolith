mod scan;
fn main() {
    let applist = scan::scanApps();
    let casklist = scan::scanCasks();
    let formulalist = scan::scanFormulaes();
    for formula in formulalist {
        match &formula.homepage {
            Some(desc) => println!("{} ", desc),
            None => println!("No description"),
        }
    }
}
