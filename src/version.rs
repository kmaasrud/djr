const DJR_VERSION: &str = env!("DJR_VERSION");

pub(crate) fn version() {
    println!("djr v{} (cli v{})", DJR_VERSION, env!("CARGO_PKG_VERSION"));
}
