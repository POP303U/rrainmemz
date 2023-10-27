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

    /*
     * -- Run this to translate the current brainfuck file to rrainmemz
     */
    //convert_bf_to_rrz(&file_content);

    // Run The Code :)
    let _ = Rrainmemz::new(file_content).run();

    Ok(())
}

#[allow(dead_code)]
fn convert_bf_to_rrz(file_contents: &String) {
    let code: Vec<u8> = file_contents.bytes().collect();
    let mut parsed_code = Vec::new();
    for i in 0..code.len() {
        match code[i] {
            b'+' => parsed_code.push(String::from("sigma,")),
            b'-' => parsed_code.push(String::from("ligma,")),
            b'>' => parsed_code.push(String::from("sideeye,")),
            b'<' => parsed_code.push(String::from("amogus,")),
            b'.' => parsed_code.push(String::from("npc,")),
            b',' => parsed_code.push(String::from("goofy,")),
            b'[' => parsed_code.push(String::from("skedaadle,")),
            b']' => parsed_code.push(String::from("skedoodle,")),
            _ => {}
        }
    }

    let mut beautified_code = String::new();
    for i in 0..parsed_code.len() {
        let str = &parsed_code[i].to_string();
        beautified_code.push_str(str);
    }
    println!("{}", beautified_code.trim())
}
