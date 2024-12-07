%start Nodes
%avoid_insert "INT"
%%

Nodes -> Result<Vec<Node>, ()>:
    Nodes Node { flattenr($1, $2) }
  | { Ok(vec![]) }
  ;

Node -> Result<Node, ()>:
      'IF' 'LPAR' Node 'RPAR' 'LPAR' Node 'RPAR' 'ELSE' 'LPAR' Node 'RPAR' {
        Ok(Node::If {
            cond: Box::new($3?),
            then_branch: Box::new($6?),
            else_branch: Box::new($10.map_err(|_| ())?)



        })
      }
    | Node 'ADD' Term {
        Ok(Node::Add{ 
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Node 'LT' Term {
        Ok(Node::Lt {
          lhs: Box::new($1?), 
          rhs: Box::new($3?)
        })
      }
    | Node 'GT' Term {
        Ok(Node::Gt {
          lhs: Box::new($1?), 
          rhs: Box::new($3?)
        })
      }
    | Term { $1 }
    ;

Term -> Result<Node, ()>:
      Term 'MUL' Factor {
        Ok(Node::Mul{  
          lhs: Box::new($1?), 
          rhs: Box::new($3?) 
        })
      }
    | Factor { $1 }
    ;

Factor -> Result<Node, ()>:
      'LPAR' Node 'RPAR' { $2 }
    | 'INT' { 
        match $1.map_err(|err| format!("Parsing Error: {}", err)) {
            Ok(s) => {
              let s = $lexer.span_str(s.span());
              match s.parse::<u64>() {
                  Ok(n_val) => Ok(Node::Number{ value: n_val }),
                  Err(_) => Err(())
              }
            }
            Err(_) => Err(())
        }
      }
    ;
%%
use crate::ast::Node;

/// Flatten `rhs` into `lhs`.
fn flattenr<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>) -> Result<Vec<T>, ()> {
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}
