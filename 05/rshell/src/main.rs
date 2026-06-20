use std::{
    io::{Read, Write, stdin, stdout},
    process::Stdio,
};

mod cmd;

const WELCOME_MSG: &str = "blazing> ";
const ERR_MSG: &str = "... Bzing error!";

fn main() {
    loop {
        print!("{WELCOME_MSG}");
        stdout().flush().expect("Output crashed.");

        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Output crashed.");

        let mut cmd_groups = buf.trim().split_terminator("|");

        let Some(frst_cmd_group) = cmd_groups.next() else {
            continue;
        };
        match cmd::CmdGroup::new(frst_cmd_group.to_string()) {
            Ok(frst_cmd_group) => {
                let Ok(mut frst_out) = frst_cmd_group.run_out(Stdio::inherit()) else {
                    println!("{ERR_MSG} Lib error - don't care");
                    continue;
                };

                match cmd_groups.next() {
                    Some(scnd_cmd_group) => match cmd::CmdGroup::new(scnd_cmd_group.to_string()) {
                        Ok(cmd_group) => {
                            let Ok(scnd_out) = cmd_group.run_wait(Stdio::from(frst_out)) else {
                                println!("{ERR_MSG} Lib error - don't care");
                                continue;
                            };
                            
                            let scnd_out = scnd_out.stdout.as_slice();
                            println!("{}", String::from_utf8_lossy(scnd_out));
                        }
                        Err(e) => {
                            println!("{ERR_MSG} {e}")
                        }
                    },
                    None => {
                        let mut frst_out_data = vec![];
                        frst_out
                            .read_to_end(&mut frst_out_data)
                            .expect("Child output crashed.");
                        println!("{}", String::from_utf8_lossy(frst_out_data.as_slice()));
                    }
                }
            }
            Err(e) => {
                println!("{ERR_MSG} {e}")
            }
        }
    }
}
