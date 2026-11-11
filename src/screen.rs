use crossterm::{ExecutableCommand, terminal::Clear, cursor::MoveTo};

pub fn draw_file(file: &Vec<String>, stdout: &mut std::io::Stdout) -> std::io::Result<()> {

    stdout.execute(Clear(crossterm::terminal::ClearType::All))?;
    stdout.execute(MoveTo(0, 0))?;

    println!("{}", file.join("\r\n"));

    Ok(())
}


