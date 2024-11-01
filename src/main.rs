use din::{evaluator, generator, lexer, parser};
use std::{env, fs, io::Write};

fn main() {
    println!(
        "
    ⠀⠀⠀⠀⠀⣼⣧⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⣼⣿⣿⣧⠀⠀⠀⠀
    ⠀⠀⠀⠾⠿⠿⠿⠿⠷⠀⠀⠀
    ⠀⠀⣼⣆⠀⠀⠀⠀⣰⣧⠀⠀
    ⠀⣼⣿⣿⣆⠀⠀⣰⣿⣿⣧⠀
    ⠾⠟⠿⠿⠿⠧⠼⠿⠿⠿⠻⠷
    picoc089: C0 intepreter, C89 compiler
    "
    );

    let strat = env::args()
        .nth(1)
        .expect("picoc089-error: no evaluation strategy given");
    println!("picoc089-info: received evaluation strategy: {strat}");

    let src = env::args()
        .nth(2)
        .expect("picoc089-error: no source file given");
    println!("picoc089-info: received source: {src}");

    let chars = fs::read(src)
        .expect("picoc089-error: should have been able to read the file")
        .iter()
        .map(|b| *b as char)
        .collect::<Vec<_>>();
    let tokens = lexer::lex(&chars);
    let tree = parser::parse(tokens).unwrap(); // C0 is a subset of C89 and share the same syntax
                                               // println!("{:?}", tree);

    match strat.as_str() {
        "interpretc0" => {
            let val = evaluator::eval(tree);
            println!("picoc089-info: evaluated: {val}");
        }
        "compilec89" => {
            let assembly = generator::gen(tree);

            let trgt = "./tmp.s";
            println!("picoc089-info: generating target: {trgt}");
            let mut f = fs::File::create(trgt).expect("picoc089-error: unable to create file");
            f.write_all(assembly.join("\n").as_bytes())
                .expect("picoc089-error: unable to write data");
        }
        _ => {
            println!("picoc089-error: unknown strategy: {:?}", strat);
            std::process::exit(1);
        }
    }
}
