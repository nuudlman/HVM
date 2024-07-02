use crate::hvm;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};
use std::collections::HashMap;
use std::slice;
use core::mem;

/// JIT compile the book and run it
pub fn run(book: &hvm::Book) {
    todo!()
}

/// JIT Compilation state
pub struct JIT {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though we aren't using this yet.
    ctx: codegen::Context,

    /// The data description, which is to data objects what `ctx` is to functions.
    data_description: DataDescription,

    /// The module, with the jit backend, which manages the JIT'd
    /// functions.
    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        todo!()
    }
}

impl JIT {
    /// Return a pointer to JIT'd machine code
    fn compile(&mut self) -> Result<*const u8, String> {
        todo!()
    }

    /// Executes the given code using the cranelift JIT compiler.
    ///
    /// Feeds the given input into the JIT compiled function and returns the resulting output.
    ///
    /// # Safety
    ///
    /// This function is unsafe since it relies on the caller to provide it with the correct
    /// input and output types. Using incorrect types at this point may corrupt the program's state.
    pub unsafe fn run_code<I, O>(&mut self, input: I) -> Result<O, String> {
        let code_ptr = self.compile()?;
        // Cast the raw pointer to a typed function pointer. This is unsafe, because
        // this is the critical point where you have to trust that the generated code
        // is safe to be called.
        let code_fn = mem::transmute::<_, fn(I) -> O>(code_ptr);
        // And now we can call it!
        Ok(code_fn(input))
    }
}