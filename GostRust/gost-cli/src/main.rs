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

fn input_path() -> anyhow::Result<std::path::PathBuf> {
    let paths = ["../../../resources/input.bin", "../resources/input.bin"];
    let p = paths.iter().find(|p| std::path::Path::new(p).exists());
    match p {
        Some(p) => Ok(std::path::PathBuf::from(p)),
        None => Err(anyhow::anyhow!("Resource not found")),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn ROTL64(x: u64, s: u64) -> u64 {
    if s == 0 {
        x
    } else {
        (x << s) | (x >> (64 - s))
    }
}

fn run() -> anyhow::Result<()> {
    gost::cc_init(0);

    let mut src = std::fs::File::open(input_path()?)?;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./output.txt")?;

    let fo = Some(&mut f);
    let so = Some(&mut src);

    gost::test_vectors::short_test_vectors(fo, so);

    let x = 0x1234_u64;
    println!("* ROTL64({:x}, 2) = {:x}", x, ROTL64(x, 4));
    println!("* ROTL64({:x}, 0) = {:x}", x, ROTL64(x, 0));
    //println!(" <<   ({}, 0) = {}", x, x << 0 | x >> (64 - 0));

    Ok(())
}
