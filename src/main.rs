use std::{env,fs,io::{self,Write}};
mod vm;
mod bytecode;
mod parse;
mod value;
mod build_in;
mod lex;
mod utils;
mod ast;
fn main() {
    let args=env::args().collect::<Vec<_>>();
    let mut runtime=vm::Execute::new();
    if args.len()>=2{
        let file=fs::read_to_string(&args[1]).expect("无法读取文件");
        runtime.exec(
            parse::Proto::parse(&mut lex::Lex::new(&file))
                .unwrap_or_else(|e| utils::show_error(&e))
        ).unwrap_or_else(|e| utils::show_error(&e));
    }else{
        loop{
            let mut line=String::new();
            print!(">>>");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line).unwrap();
            runtime.exec(
                parse::Proto::parse(&mut lex::Lex::new(&line))
                    .unwrap_or_else(|e| utils::show_error(&e))
            ).unwrap_or_else(|e| utils::show_error(&e));
            println!();
        }
    }
}
#[cfg(test)]
mod tests{
}