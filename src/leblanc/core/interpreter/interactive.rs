use std::io::Write;
use crate::{CompilationMode, compile};
use crate::leblanc::core::bytecode::LeblancBytecode;
use crate::leblanc::core::interpreter::instructions::Instruction;

static MAIN_MODE: bool = true;

pub fn start() {
    loop {
        let mut s = retrieve_input();
        if MAIN_MODE {
            s = wrap_in_main(s);
        }
        println!("compiled:\n{}", s);

        let fabric = compile(s, CompilationMode::Realtime);
        let mut bytecode = LeblancBytecode::from(fabric.bytecode);
        for mut function in bytecode.body().functions() {
            let mut instructs: Vec<Instruction> = vec![];
            function.instruction_lines().into_iter().map(|line| line.to_instructions()).for_each(|mut l| instructs.append(&mut l));
            println!("instructs: {:#?}", instructs);
        }
    }








}

fn retrieve_input() -> String {
    let mut dc = DelimiterCount::new();
    let mut line = String::new();

    let mut final_line = String::new();
    let mut console = ">>> ";
    while !dc.balanced() || final_line.is_empty() || final_line.ends_with(";;") {
        final_line = final_line.replace("...", "");
        print!("{}", console);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        line.pop().unwrap();
        line.lines().map(|l| l.chars()).for_each(|c| c.into_iter().for_each(|ch| dc.check(ch)));
        final_line += &line;
        std::io::stdout().flush().unwrap();
        line = String::new();
        console = "... ";
    }


    final_line
}

#[derive(Debug)]
struct DelimiterCount {
    bracket: i16,
    parenthesis: i16,
    brace: i16,
}

impl DelimiterCount {
    pub fn new() -> DelimiterCount {
        DelimiterCount {
            bracket: 0,
            parenthesis: 0,
            brace: 0
        }
    }

    pub fn check(&mut self, ch: char) {
        match ch {
            '{' => self.brace += 1,
            '(' => self.parenthesis += 1,
            '[' => self.bracket += 1,
            '}' => self.brace -= 1,
            ')' => self.parenthesis -= 1,
            ']' => self.bracket -=1,
            _ => {}
        };
    }

    pub fn balanced(&self) -> bool {
        self.bracket == 0 && self.parenthesis == 0 && self.brace == 0
    }
}

pub fn wrap_in_main(string: String) -> String {
    "func main() {\n".to_owned() + "\t" + &string.replace(";;", ";\n\t") + "\n}"
}