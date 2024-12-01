%start Program
%%
Program -> Result<u64, ()>:
      Stmt { $1 }
    ;

Stmt -> Result<u64, ()>:
      VarDecl { $1 }
    | VarChang { $1 }
    | Expr { $1 }
    ;

VarDecl -> Result<u64, ()>:
      'VAR' Var '=' Expr 
      { 
        let var_value = $4?;
        let var_name = $lexer.span_str($2?.span());
        store_variable(var_name , var_value);
        Ok(var_value)
      }
    ;   

VarChang -> Result<u64, ()>:
        Var '=' Expr
        {
          let var_name = $lexer.span_str($1?.span());
          match get_variable(var_name) {
            Some(_val) => {
              let var_new_value = $3?;
              redefine_variable_value(var_name, var_new_value);
              Ok(var_new_value)
            },
            None => {
                eprintln!("A variavel '{}' não foi definida", var_name);
                Err(())
            }
          }
        }
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

fn redefine_variable_value(name: &str, new_value: u64) {
  let mut vars = VARIABLES.lock().unwrap();
  vars.entry(name.to_string())
      .and_modify(|value| *value = new_value);
}

fn get_variable(name: &str) -> Option<u64> {
  let vars = VARIABLES.lock().unwrap();
  vars.get(name).copied()
}
