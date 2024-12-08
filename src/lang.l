%%
[0-9]+ "INT"
\+ "ADD"
\* "MUL"
\( "LPAR"
\) "RPAR"
; ";"
= "ASSIGN"
let "LET" 
writeln "WRITELN" 
[a-zA-Z0-9_]+ "ID"
[\t\n ]+ ;
//[^\n]*?$ ;
