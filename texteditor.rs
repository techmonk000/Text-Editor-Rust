use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::env;

fn main() -> io::Result<()> {
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return Ok(());
    }
    let filename = &args[1];

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.execute(terminal::Clear(ClearType::All))?;
    handle.flush()?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    print!("{}", buffer);

    handle.execute(cursor::MoveTo(0, 0))?;
    handle.flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    file.write_all(input.as_bytes())?;

    Ok(())
}
