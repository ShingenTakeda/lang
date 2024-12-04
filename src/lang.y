%start Program
%%
Program -> Result<u64, ()>:
      OptionalNewlines StmtList OptionalNewlines { $2 }
    ;

StmtList -> Result<u64, ()>:
      Stmt { $1 }
    | StmtList 'NEWLINE' OptionalNewlines Stmt { 
        $4
      }
    ;

OptionalNewlines -> ():
      /* empty */ { }
    | 'NEWLINE' OptionalNewlines { }
    ;

Stmt -> Result<u64, ()>:
      FunctionDef { $1 }
    | VarDecl { $1 }
    | VarChang { $1 }
    | ExprStmt { $1 }
    ;

ExprStmt -> Result<u64, ()>:
      Expr { $1 }
    ;

FunctionDef -> Result<u64, ()>:
    'FN' Identifier '{' OptionalNewlines StmtList OptionalNewlines '}' 
    {
        if let Ok(id_token) = $2 {
            let name = $lexer.span_str(id_token.span());
            if let Ok(val) = $5 {
                _ = store_function(name, &val.to_string());
                Ok(0)
            } else {
                eprintln!("Erro ao processar corpo da função");
                Err(())
            }
        } else {
            eprintln!("Erro ao processar nome da função");
            Err(())
        }
    }
    ;

VarDecl -> Result<u64, ()>:
      'VAR' Identifier '=' Expr 
      { 
        let var_value = $4?;
        let var_name = $lexer.span_str($2?.span());
        store_variable(var_name , var_value);
        Ok(var_value)
      }
    ;   

VarChang -> Result<u64, ()>:
    Identifier '=' Expr
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
    | Reference { $1 }
    | '(' Expr ')' { $2 }
    ;

Reference -> Result<u64, ()>:
    Identifier RefKind { 
        let name = $lexer.span_str($1?.span());
        match $2? {
            RefType::Variable => {
                match get_variable(name) {
                    Some(val) => Ok(val),
                    None => {
                        eprintln!("A variavel '{}' não foi definida", name);
                        Err(())
                    }
                }
            },
            RefType::Function => {
                execute_function(name)
            }
        }
    }
    ;

RefKind -> Result<RefType, ()>:
      /* empty */ { Ok(RefType::Variable) }
    | '(' ')' { Ok(RefType::Function) }
    ;

Identifier -> Result<lrlex::DefaultLexeme, ()>:
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
use crate::lang_l;
use crate::lang_y;

enum RefType {
    Variable,
    Function
}

struct Function {
    body: String,
    parsed_ast: Option<Result<u64, ()>>,
}

static FUNCTIONS: Lazy<Mutex<HashMap<String, Function>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

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

fn store_function(name: &str, body: &str) -> Result<(), ()> {
    let binding = lang_l::lexerdef();
    let lexer = binding.lexer(body);
    let (ast, errs) = lang_y::parse(&lexer);
    
    if !errs.is_empty() {
        for e in errs {
            eprintln!("Erro ao parsear função: {}", e.pp(&lexer, &lang_y::token_epp));
        }
        return Err(());
    }

    let function = Function {
        body: body.to_string(),
        parsed_ast: ast,
    };

    let mut funcs = FUNCTIONS.lock().unwrap();
    funcs.insert(name.to_string(), function);
    Ok(())
}

fn execute_function(name: &str) -> Result<u64, ()> {
    let funcs = FUNCTIONS.lock().unwrap();
    if let Some(func) = funcs.get(name) {
        if let Some(result) = &func.parsed_ast {
            *result
        } else {
            eprintln!("Função '{}' tem AST inválida", name);
            Err(())
        }
    } else {
        eprintln!("Função '{}' não foi definida", name);
        Err(())
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
