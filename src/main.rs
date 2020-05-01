use std::io::{self,Write};
use std::process::Command;

fn main() -> std::io::Result<()> {
//---------------------------------2------------------------------------------

let stdin = io::stdin();



let mut user_input = String::with_capacity(256);

print!(">");
let _stdout= io::stdout().flush();


stdin.read_line(&mut user_input)?;






//print!("{}",user_input);

//-----------3---------------------------------------------

let status = Command::new("ls").status();
//print!("fdsf");
//---------------------------------------------------4-------------------------------------------
//
// status nous force à recuperer les resultat directement , elle redérige les résultats de notre nouveau processus à la sortie standart et tue le processus fils 

//-----------------------------------------------------------5----------------------------------------------------------------
//Que fait votre programme pendant que son enfant s’exécute ?
// il attend que le fils se temrine 

//----------6------------------------

let st=Command::new("ls").arg("-l").status();




// `?` sert à « propager l'erreur » dans la fonction appellante
// c'est mieux que de crash avec un unwrap ou expect ;)
Ok(())
}