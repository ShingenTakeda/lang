%%
[a-Za-Z0-9_]+ "ID"
[0-9]+ "INT"
\+ "ADD"
\- "SUB"
\* "MUL"
\/ "DIV"
\( "LPAR"
\) "RPAR"
//[^\n]*?$ ;
[\t\n ]+ ;
