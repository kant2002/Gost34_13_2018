use std::process::ExitCode;
fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> anyhow::Result<()> {
    gost::cc_init(0);

    let mut src = std::fs::File::open("../../../resources/input.bin")?;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./output.txt")?;

    let fo = Some(&mut f);
    let so = Some(&mut src);

    gost::test_vectors::short_test_vectors(fo, so);

    Ok(())
}
