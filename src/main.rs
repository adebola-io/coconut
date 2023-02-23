mod lib;

fn main() {
    if let Err(e) = run() {
        lib::terminal::error(e);
    }
}

fn run() -> lib::Fallible<()> {
    lib::build_config(std::env::current_dir()?)?.run()?;
    Ok(())
}
