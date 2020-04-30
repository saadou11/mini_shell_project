use std::io::{self,Write};

fn main() -> std::io::Result<()> {
let stdin = io::stdin();;
let mut user_input = String::with_capacity(256);
print!(">");
let stdout= io::stdout().flush();

// On prend une référence mutable
stdin.read_line(&mut user_input)?;
print!("{}",user_input);

// `?` sert à « propager l'erreur » dans la fonction appellante
// c'est mieux que de crash avec un unwrap ou expect ;)
Ok(())
}