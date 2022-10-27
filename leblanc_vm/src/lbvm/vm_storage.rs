use crate::lbvm::method::Method;
use crate::lbvm::object::Object;

pub enum Constant {
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(Object)
}

pub enum VmItem {

}



#[derive(Default)]
pub struct VmStorage {
    pub(crate) constants: Vec<Constant>,
    pub(crate) methods: Vec<Method>,
    pub(crate) items: Vec<VmItem>
}