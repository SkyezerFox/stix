use fluxc_ast::UnaryExpr;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for UnaryExpr {
    fn translate<'a>(&self, _ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
