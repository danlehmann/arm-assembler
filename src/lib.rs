mod ast;
mod encode;
mod error;
mod lexer;
mod parser;

pub use ast::{Cpu, Isa};
pub use error::AsmError;

/// Configuration for the assembler.
#[derive(Debug, Clone)]
pub struct AsmConfig {
    /// Default instruction set. Can be overridden by `.thumb` / `.arm` directives.
    pub default_isa: Isa,
}

/// A named section in the output.
#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub data: Vec<u8>,
}

/// A symbol (label) defined in the assembly.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub section_index: usize,
    pub offset: u32,
    pub global: bool,
}

/// Output of the assembler.
#[derive(Debug, Clone)]
pub struct AsmOutput {
    pub sections: Vec<Section>,
    pub symbols: Vec<Symbol>,
}

impl AsmOutput {
    /// Convenience: get the `.text` section bytes.
    pub fn text_bytes(&self) -> &[u8] {
        self.sections
            .iter()
            .find(|s| s.name == ".text")
            .map(|s| s.data.as_slice())
            .unwrap_or(&[])
    }
}

/// Assemble ARM source code into machine code.
pub fn assemble(code: &str, config: &AsmConfig) -> Result<AsmOutput, AsmError> {
    let tokens = lexer::tokenize(code)?;
    let statements = parser::parse(&tokens)?;
    encode::assemble(&statements, config)
}
