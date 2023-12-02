#[derive(argh::FromArgs)]
/// Day X Challenge
struct Options {}

fn main() -> color_eyre::Result<()> {
    let _opts: Options = argh::from_env();

    Ok(())
}
