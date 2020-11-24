use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: hexdump001 file");
        return Ok(());
    }

    let file = &args[1];
    let buf = std::fs::read_to_string(file)?;

    let mut address = 0u64;
    for line in buf.as_bytes().chunks(16) {
        print!("[0x{:08x}]", address);

        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();

        address += 16;
    }

    Ok(())
}
