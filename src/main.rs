use std::io::{self,Write};
use std::process::{Command, Stdio};
use std::fmt;
use std::thread;
#[derive(Clone, Debug)]
pub struct Job{
    pub job_id : u32,
    pub job_command : String,
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}   {}", self.job_id, self.job_command)
    }
} 


fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    //let mut jobs = Jobs{table : Vec::new(), count : 0};
    let mut jobs =Vec::new();
    let mut user_input = String::with_capacity(256);

loop {
    let mut handle = stdout.lock();
    handle.write_all(b"> ")?;
    handle.flush()?;

    let mut user_input = String::with_capacity(256);
    stdin.read_line(&mut user_input)?;
    user_input = user_input.trim_end().trim_end().to_string();

    // quit terminal
    if user_input == "quit" {
        break;
    }

    let command = split_command_with_espace(&user_input);
    if command.starts_with("minips"){
        for i in 0..(jobs.len()-1){
            println!("{}",jobs[i]);
        }   
    }

    if command.starts_with("back"){
        // ------------------------------ back with pipe command--------------------------------
        if command.contains("|"){
            let commands : Vec<&str> = command.split("|").collect();  
            let first_command = split_command_with_espace(commands[0]);
            let first_args : Vec<&str> = first_command.split(" ").collect();

            let mut parent_command = Command::new(first_args[1]).args(&first_args[2..])
                                    .stdout(Stdio::piped())
                                    .spawn()
                                    .expect("failed to execute command");

            for index in 1 .. (commands.len() - 1) {

                let center_command = split_command_with_espace(commands[index]);
                let center_args : Vec<&str> = center_command.split(" ").collect();
                parent_command =  Command::new(center_args[0]).args(&center_args[1..])
                                .stdin(parent_command.stdout.unwrap())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("failed to execute command");
        }

            let last_commmand = split_command_with_espace(commands[commands.len() - 1]);
            let last_args : Vec<&str> = last_commmand.split(" ").collect();

            let mut child = Command::new(last_args[0]).args(&last_args[1..])
                        .stdin(parent_command.stdout.unwrap())
                        .spawn()
                        .expect("failed to execute command");
            
            let job = Job{job_id : child.id(), job_command : command.clone()}; 
            jobs.push(job);
            match child.try_wait() {
                Ok(Some(res)) => println!("[{}] {} {:?}",jobs.len(), child.id(),command ),
                //println!("la  [{}] {} {:?}",jobs.table.len(), child.id(), command),
                Ok(None) => {; 
                    println!("[{}] {} {:?}",jobs.len(), child.id(), command);
                }
                Err(e) => println!("error attempting to wait: {}", e),   
        }

    
        }
        else 
        {
            //-----------------back simple command----------------------------
                         //let child : std::process::Child;
            let args : Vec<&str> = command.split(" ").collect();
            let mut child = Command::new(args[1]).args(&args[2..]).spawn()?;         

            let job = Job{job_id : child.id(), job_command : command.clone()}; 
            jobs.push(job);
            match child.try_wait() {
                Ok(_res) => println!("[{}] {} {:?}",jobs.len(), child.id(),args[1] ),
                //println!("la  [{}] {} {:?}",jobs.table.len(), child.id(), command),
                Ok(None) => {
                    println!("[{}] {} {:?}",jobs.len(), child.id(), args[1]);
                }
                Err(e) => println!("error attempting to wait: {}", e),   
        }

    }
}



    else {
        // ----------------------------simple pipe command----------------------------
        if command.contains("|"){
            let commands : Vec<&str> = command.split("|").collect();  
            let first_command = split_command_with_espace(commands[0]);
            let first_args : Vec<&str> = first_command.split(" ").collect();

            let mut parent_command = Command::new(first_args[0]).args(&first_args[1..])
                                    .stdout(Stdio::piped())
                                    .spawn()
                                    .expect("failed to execute command");

            for index in 1 .. (commands.len() - 1) {

                let center_command = split_command_with_espace(commands[index]);
                let center_args : Vec<&str> = center_command.split(" ").collect();
                parent_command =  Command::new(center_args[0]).args(&center_args[1..])
                                .stdin(parent_command.stdout.unwrap())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("failed to execute command");
        }

            let last_commmand = split_command_with_espace(commands[commands.len() - 1]);
            let last_args : Vec<&str> = last_commmand.split(" ").collect();

            let status = Command::new(last_args[0]).args(&last_args[1..])
                        .stdin(parent_command.stdout.unwrap())
                        .status()
                        .expect("failed to execute command");

            if !status.success(){
                println!("failed to execute");
            }
        }
        else
        //------------------------------------normal command----------------------------------------------
        {
            let args : Vec<&str> = command.split(" ").collect();
            let status = Command::new(args[0]).args(&args[1..]).status() .expect("failed to execute command");         
            if !status.success(){
                    println!("failed to execute");
            }
        }
    }
};
Ok(())
}

fn split_command_with_espace(s : &str) -> String{
    let mut index = 0;
    let mut ret : String = "".to_string();
    for i in s.chars(){ 
        if i != ' '{
            ret = s[index..].to_string();
            break;
        }
        index += 1;
    }

    let str_len = s.len();
    let mut index = 0;
    while index < (str_len - 2) {
        if ret.get(index .. (index + 2)) == Some("  ") {
            ret.remove(index + 1);
        }else {
            index += 1;
        }
    }

    let ret = ret.trim_end();

    ret.to_string()
}