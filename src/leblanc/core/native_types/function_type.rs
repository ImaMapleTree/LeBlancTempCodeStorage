use std::collections::{BTreeSet, HashMap};
use crate::leblanc::core::bytecode::function_bytes::FunctionBytecode;
use crate::leblanc::core::internal::methods::internal_math::_internal_add_double_;
use crate::leblanc::core::leblanc_argument::number_argset;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::method_store::MethodStore;
use crate::leblanc::core::method_tag::MethodTag;
use crate::leblanc::core::native_types::base_type::{base_methods, ToLeblanc};
use crate::LeBlancType;


