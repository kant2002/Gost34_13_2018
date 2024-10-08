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
    println!("Hello, world!");
    let mut r0: [u8; 17] = [0; 17];
    r0.iter_mut()
        .take(15)
        .enumerate()
        .for_each(|x| *x.1 = x.0 as u8);
    r0[15] = 30;
    r0[16] = 31;
    println!("{:?}", r0);

    gost::init(0);
    r0.iter_mut().for_each(|x| *x = gost::rnext());
    println!("{:?}", r0);

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
