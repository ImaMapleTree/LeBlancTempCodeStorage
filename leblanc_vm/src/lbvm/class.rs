use crate::lbvm::method::Method;
use crate::leblanc::copystring::CopyString;

pub struct LbvmClass {
    /// Name of Class, used for debug and for fallback in unsafe API
    pub(crate) name: CopyString,
    /// Class methods used for invocation
    pub(crate) methods: Vec<Method>,










}