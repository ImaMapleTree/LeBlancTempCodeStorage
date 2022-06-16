pub mod stub_compiler;
pub mod full_compiler;
pub mod full_reader;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CompilationMode {
    Full,
    StubFile,
    ByteCode,
    Realtime
}