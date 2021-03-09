use crate::expression::Expr;

peg::parser!(pub grammar parser() for str {
    pub rule function() -> (String, Vec<String>, String, Vec<Expr>)
        = [ ' ' | '\t' | '\n' ]* name:identifier() _
        params:((_ i:identifier() _ {i}) ** ",") _
        "->" returns:(_ i:identifier() _ {i}) _
        "{" _ "\n"
        stmts:statements()
        _ "}" _ "\n"
        { (name, params, returns, stmts) }

    rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    rule statement() -> Expr
        = _ e:expression() _ "\n" { e }

    rule expression() -> Expr
        = if_else()
        / loop_block()
        / assignment()
        / binary_op()

    rule if() -> Expr
        = "if" _ e:expression() _ "{" then_body:statements() _ "}"
        { Expr::If(Box::new(e), then_body) }

    rule if_else() -> Expr
        = "if" _ e:expression() _ "{" _
        then_body:statements() _ "}" _ "else" _ "{" _
        else_body:statements() _ "}"
        { Expr::IfElse(Box::new(e), then_body, else_body) }

    rule loop_block() -> Expr
        = "loop" _ e:expression()? _ "{" _ "\n"
        loop_body:statements() _ "}"
        { Expr::Loop(Box::new(e), loop_body) }

    rule assignment() -> Expr
        = i:identifier() _ "=" _ e:expression() {Expr::Assign(i, Box::new(e))}

    rule binary_op() -> Expr = precedence!{
        a:@ _ "==" _ b:(@) { Expr::Eq(Box::new(a), Box::new(b)) }
        a:@ _ "!=" _ b:(@) { Expr::Ne(Box::new(a), Box::new(b)) }
        a:@ _ "<"  _ b:(@) { Expr::Lt(Box::new(a), Box::new(b)) }
        a:@ _ "<=" _ b:(@) { Expr::Le(Box::new(a), Box::new(b)) }
        a:@ _ ">"  _ b:(@) { Expr::Gt(Box::new(a), Box::new(b)) }
        a:@ _ ">=" _ b:(@) { Expr::Ge(Box::new(a), Box::new(b)) }
        --
        a:@ _ "+" _ b:(@) { Expr::Add(Box::new(a), Box::new(b)) }
        a:@ _ "-" _ b:(@) { Expr::Sub(Box::new(a), Box::new(b)) }
        --
        a:@ _ "*" _ b:(@) { Expr::Mul(Box::new(a), Box::new(b)) }
        a:@ _ "/" _ b:(@) { Expr::Div(Box::new(a), Box::new(b)) }
        --
        i:identifier() _ args:((_ e:expression() _ {e}) ** ",") { Expr::Call(i, args) }
        i:identifier() { Expr::Identifier(i) }
        l:literal() { l }
    }

    rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule literal() -> Expr
        = n:$(['0'..='9']+) { Expr::Literal(n.to_owned()) }
        / "&" i:identifier() { Expr::GlobalDataAddr(i) }

    rule _() =  quiet!{[' ' | '\t' | '\n']*}
    
});

