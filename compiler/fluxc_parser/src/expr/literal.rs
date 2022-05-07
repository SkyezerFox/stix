use pest::iterators::Pair;

use fluxc_ast::{Node, Literal};
use fluxc_errors::CompilerError;

use crate::{Parse, Rule, Context};

impl Parse for Literal {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
