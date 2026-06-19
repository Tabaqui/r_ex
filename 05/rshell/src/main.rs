use std::{
    io::{Read, Write, stdin, stdout},
    process::{Child, ChildStdout, Command, Stdio},
};

const WELCOME_MSG: &str = "blazing> ";
const ERR_MSG: &str = "... Bzing error!";

fn main() {
    loop {
        print!("{WELCOME_MSG}");
        if let Err(e) = stdout().flush() {
            println!("{ERR_MSG} {e}");
            continue;
        }

        let mut buf = String::new();
        if let Err(e) = stdin().read_line(&mut buf) {
            println!("{ERR_MSG} {e}");
            continue;
        }

        let mut cmd_groups = buf.split_terminator("|");

        let Some(frst_cmd_group) = cmd_groups.next() else {
            continue;
        };

        let mut frst_cmd_group = frst_cmd_group.trim().split_ascii_whitespace();

        let Some(frst_cmd) = frst_cmd_group.next() else {
            println!("{ERR_MSG} Command take fault.");
            continue;
        };

        let frst_args = frst_cmd_group;

        let frst_prc = Command::new(frst_cmd)
            .args(frst_args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .spawn();

        if let Err(e) = frst_prc {
            println!("{ERR_MSG} {e}");
            continue;
        }

        let frst_prc = frst_prc.unwrap();
        let Some(mut frst_out) = frst_prc.stdout else {
            println!("{ERR_MSG} Command out fault");
            continue;
        };

        let scnd_cmd_group = cmd_groups.next();
        match scnd_cmd_group {
            Some(scnd_cmd_group) => {
                let mut scnd_cmd_group = scnd_cmd_group.trim().split_ascii_whitespace();
                let Some(scnd_cmd) = scnd_cmd_group.next() else {
                    println!("{ERR_MSG} Command take fault.");
                    continue;
                };
                let scnd_args = scnd_cmd_group;

                let scnd_prc = Command::new(scnd_cmd)
                    .args(scnd_args)
                    .stdin(Stdio::from(frst_out))
                    .stdout(Stdio::piped())
                    .spawn();

                if let Err(e) = scnd_prc {
                    println!("{ERR_MSG} {e}");
                    continue;
                }

                let scnd_prc = scnd_prc.unwrap();
                let scnd_out = scnd_prc
                    .wait_with_output()
                    .expect("{ERR_MSG} Take process output fault.");
                let scnd_out = scnd_out.stdout.as_slice();

                println!("{}", String::from_utf8_lossy(scnd_out));
            }
            None => {
                let mut frst_out_data = vec![];
                let Ok(_) = frst_out.read_to_end(&mut frst_out_data) else {
                    ("{ERR_MSG} Read process output fault.");
                    continue;
                };
                println!("{}", String::from_utf8_lossy(frst_out_data.as_slice()));
            }
        }
    }
}

struct CmdGroup {
    value: String,
    // frst_cmd_grp: String,
    frst_cmd: String,
    frst_args: Option<String>,
}

impl CmdGroup {
    fn new(value: String) -> Self {
        let mut g = value.split_ascii_whitespace();
        let f = g.next().unwrap().to_string();
        let f_a = g.next().map(|s| s.to_string());
        Self {
            value,
            frst_cmd: f,
            frst_args: f_a,
        }
    }

    fn run(self, stdin: Stdio) -> ChildStdout {
        let mut c = Command::new(self.frst_cmd);
        c.args(self.frst_args).stdout(Stdio::piped());

        let c = c.stdin(stdin);
        let a = c.spawn().unwrap().stdout;

        a.unwrap()
    }
}

