%start Program
%%
Program -> Result<u64, ()>:
      Stmt { $1 }
    ;

Stmt -> Result<u64, ()>:
      Var '=' Expr 
      { 
        let var_value = $3?;
        let var_name = $lexer.span_str($1?.span());
        store_variable(var_name , var_value);
        Ok(var_value)
      }
    | Expr { $1 }
    ;

Expr -> Result<u64, ()>:
      Term { $1 }
    | Expr '+' Term { Ok($1? + $3?) }
    ;

Term -> Result<u64, ()>:
      Factor { $1 }
    | Term '*' Factor { Ok($1? * $3?) }
    ;

Factor -> Result<u64, ()>:
      Num { $1 }
    | Var 
      { 
        let name = $lexer.span_str($1?.span());
        match get_variable(name) {
            Some(val) => Ok(val),
            None => {
                eprintln!("A variavel '{}' não foi definida", name);
                Err(())
            }
        }
      }
    | '(' Expr ')' { $2 }
    ;

Var -> Result<lrlex::DefaultLexeme, ()>:
      'ID' { $1.map_err(|_| ()) }
    ;

Num -> Result<u64, ()>:
      'INT'
      {
          let v = $1.map_err(|_| ())?;
          parse_int($lexer.span_str(v.span()))
      }
    ;

%%
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static VARIABLES: Lazy<Mutex<HashMap<String, u64>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

fn parse_int(s: &str) -> Result<u64, ()> {
    match s.parse::<u64>() {
        Ok(val) => Ok(val),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}

fn store_variable(name: &str, value: u64) {
    let mut vars = VARIABLES.lock().unwrap();
    vars.insert(name.to_string(), value);
}

fn get_variable(name: &str) -> Option<u64> {
    let vars = VARIABLES.lock().unwrap();
    vars.get(name).copied()
}
