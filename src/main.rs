mod rrainmemz;
use {rrainmemz::Rrainmemz, std::env, std::fs, std::io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path)?;

    // Run The Code :)
    let _ = Rrainmemz::new(file_content).run();

    Ok(())
}
