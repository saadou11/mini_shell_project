use std::io::{self,Write};
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
//-----------2---------------------------------------------

let stdin = io::stdin();
let mut user_input = String::with_capacity(256);

print!(">");
let _stdout = io::stdout().flush();
stdin.read_line(&mut user_input)?;

//-----------3---------------------------------------------

// let status = Command::new("ls").status();

//let statys_with_argument = Command::new("ls")
                                   // .arg("-l")
                                   // .status();

//-----------4---------------------------------------------
let mut first_command = Command::new("ls")
                                    .stdout(Stdio::piped())
                                    .spawn()?;

if let Some(du_output) = first_command.stdout.take() {
let mut second_command = Command::new("grep")
                                    .arg("src")
                                    .stdin(du_output)
                                    .stdout(Stdio::piped())
                                    .spawn()?;

 let my_stdout = second_command.wait_with_output()?;

 println!("{}", String::from_utf8(my_stdout.stdout).unwrap());
}
//-----------5---------------------------------------------




Ok(())
}