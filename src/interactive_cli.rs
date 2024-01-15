use std::io::{stdin, stdout, Write};

pub fn enter_interactive_mode() -> Result<(), std::io::Error> {
    println!("\nFe Interactive JIT v0.1");

    let mut instruction: String = String::new();
    loop {
        print!("Fe>> ");
        stdout().flush()?;

        stdin().read_line(&mut instruction)?;
        print!("{}", instruction);
        stdout().flush()?;

        if instruction.trim() == "exit()" {
            break;
        }

        instruction.clear();
    }

    Ok(())
}
