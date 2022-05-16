use fluxc_ast::Conditional;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for Conditional {
	fn translate<'a>(&self, ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
		todo!()
	}
}
