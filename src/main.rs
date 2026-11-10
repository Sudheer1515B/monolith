mod scan;
fn main() {
    let applist = scan::scanApps();
    let casklist = scan::scanCasks();
    let formulalist = scan::scanFormulaes();
    for formula in formulalist {
        println!("{}", formula.name);
    }
    for cask in casklist {
        println!("{}", cask.token);
    }
}
