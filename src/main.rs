fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s = String::new();
    print!("Please enter video link/URL: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    use std::process::Command;
    let _output = if cfg!(target_os = "windows") {
        Command::new("destreamer.cmd")
                .arg("-i")
                .arg(s)
                .arg("--format mp4")
                .spawn()
                .expect("failed to execute process")
    } else {
        Command::new("./destreamer.sh")
                .arg("-i")
                .arg(s)
                .arg("--format mp4")
                .spawn()
                .expect("failed to execute process")
    };
}