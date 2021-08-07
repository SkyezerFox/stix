use std::error::Error;

use cranelift::{codegen, frontend::FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Module};

use styxc_ast::{Node, NodeKind};

/// The basic JIT class.
pub struct IrTranslator {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    ctx: codegen::Context,

    /// The data context, which is to data objects what `ctx` is to functions.
    data_ctx: DataContext,

    /// The module, with the jit backend, which manages the JIT'd
    /// functions.
    module: JITModule,
}

impl Default for IrTranslator {
    fn default() -> Self {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }
}

impl IrTranslator {
    fn translate_func(
        &self,
        name: String,
        params: Vec<String>,
        ret: String,
        statements: Vec<Node>,
    ) {
    }

    /// Translate an expression into LLVM IR.
    fn translate_expr(&self, node: Node) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    /// Compile a root vector of expressions.
    fn compile(&self, root: Node) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub fn compile_ir(input: Node) -> Result<(), ()> {
    let mut ir = IrTranslator::default();
    Ok(())
}
