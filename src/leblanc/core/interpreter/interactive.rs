use crate::{CompilationMode, compile};

pub fn start() {
    while true {
        let s = retrieve_input();
        compile(s, CompilationMode::Realtime);

    }








}

fn retrieve_input() -> String {
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    return line;
}