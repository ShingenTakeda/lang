use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::{env, io::{self, BufRead, Write}};

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the
// module name will be `calc_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("lang.l");
// Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the
// module name will be `calc_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("lang.y");

fn process_input(input: &str, lexerdef: &lrlex::LRNonStreamingLexerDef<lrlex::DefaultLexerTypes>) {
    let lexer = lexerdef.lexer(input);
    let (res, errs) = lang_y::parse(&lexer);

    for e in errs {
        println!("{}", e.pp(&lexer, &lang_y::token_epp));
    }

    match res {
        Some(Ok(r)) => println!("Resultado: {:?}", r),
        _ => eprintln!("Não foi possível avaliar o código."),
    }
}

fn find_file(filename: &str) -> Option<PathBuf> {
    // Tenta no diretório atual
    let mut path = env::current_dir().unwrap();
    path.push(filename);
    if path.exists() {
        println!("Arquivo encontrado em: {}", path.display());
        return Some(path);
    }

    // Tenta na pasta src
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.push(filename);
    if path.exists() {
        println!("Arquivo encontrado em: {}", path.display());
        return Some(path);
    }

    // Tenta no diretório do projeto (um nível acima)
    let mut path = env::current_dir().unwrap();
    path.pop();  // Sobe um nível
    path.push(filename);
    if path.exists() {
        println!("Arquivo encontrado em: {}", path.display());
        return Some(path);
    }

    println!("Arquivo não encontrado em nenhum diretório conhecido");
    None
}

// Para rodar inline usar "cargo run" e para rodar o arquivo txt usar "cargo run nome_do_arquivo.txt"
fn main() {
    let lexerdef = lang_l::lexerdef();
    let args: Vec<String> = env::args().collect();

    let args_len = args.len();
    match args_len {
        1 => {
            let stdin = io::stdin();
            println!("Modo interativo. Digite 'exit' para sair.");
            loop {
                print!(">>> ");
                io::stdout().flush().ok();
                
                let mut line = String::new();
                if let Ok(n) = stdin.lock().read_line(&mut line) {
                    if n == 0 || line.trim() == "exit" {
                        break;
                    }
                    if !line.trim().is_empty() {
                        process_input(&line, &lexerdef);
                    }
                }
            }
        },
        2 => {
            if let Some(file_path) = find_file(&args[1]) {
                let mut file = match File::open(&file_path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Erro ao abrir o arquivo: {}", e);
                        std::process::exit(1);
                    }
                };
    
                let mut input = String::new();
                if let Err(e) = file.read_to_string(&mut input) {
                    eprintln!("Erro ao ler o arquivo: {}", e);
                    std::process::exit(1);
                }
    
                process_input(&input, &lexerdef);
            } else {
                eprintln!("Arquivo não encontrado");
                std::process::exit(1);
            }
        }
        _ => {
            std::process::exit(1);
        }
    }
}
