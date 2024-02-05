use crate::parser::{Function, Program};

enum Extern {
    GetStdHandle,
    WriteConsole,
    ExitProcess,
}


struct AssemblyCode {
    externs: Vec<Extern>,
    assembly: String,
}

impl AssemblyCode {
    pub fn new() -> Self {
        Self {
            externs: Vec::new(),
            assembly: String::new(),
        }
    }
}

#[macro_export]
macro_rules! add_str {
    ($first:expr $(, $rest:expr)*) => {
        {
            $(
                $first.assembly.push_str(&String::from($rest));
            )*
        }
    };
}
pub struct Generator {}

impl Generator {
    pub fn generate(prog: Program) {
        let mut generated_code = AssemblyCode::new();
        let function = prog.function_declaration;
        Self::generate_function(&mut generated_code, function);
    }

    fn generate_function(generated_code: &mut AssemblyCode, function: Function) {
        add_str!(generated_code, "section .text:\n");
        add_str!(generated_code, "global ", &function.name, '\n');
        add_str!(generated_code, &function.name, ":");
        println!("{}", generated_code.assembly);
    }
}
