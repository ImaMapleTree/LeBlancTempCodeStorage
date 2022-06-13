pub mod stub_compiler;
mod full_compiler;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CompilationMode {
    Full,
    StubFile,
    ByteCode,
    Realtime
}