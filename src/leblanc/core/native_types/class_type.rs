use std::fmt::{Display, Formatter};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::leblanc::core::leblanc_context::VariableContext;
use crate::leblanc::core::leblanc_default_data::unsafe_empty_members;
use crate::leblanc::core::leblanc_object::{LeBlancObject, LeBlancObjectData};
use crate::leblanc::core::method::Method;
use crate::leblanc::core::native_types::base_type::base_methods;
use crate::leblanc::rustblanc::copystring::{CopyString, CopyStringable};
use crate::leblanc::rustblanc::strawberry::Strawberry;
use crate::leblanc::core::native_types::LeBlancType;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ClassMeta {
    pub name: CopyString,
    pub supertypes: Vec<ClassMeta>,
    pub parse_id: u32,
}

pub fn leblanc_object_custom(meta: ClassMeta) -> LeBlancObject {
    let base_methods = base_methods();
    let name = meta.name;
    LeBlancObject::new(
        LeBlancObjectData::Class(Box::new(meta)),
        LeBlancType::Class(name),
        base_methods,
        unsafe_empty_members(),
        VariableContext::empty(),
    )
}

impl ClassMeta {
    pub fn new<'a>(name: CopyString, supertypes: Vec<ClassMeta>, parse_id: u32) -> ClassMeta {
        ClassMeta {
            name,
            supertypes,
            parse_id
        }
    }

    pub fn default<'a>(name: String, parse_id: u32) -> ClassMeta {
        ClassMeta {
            name: CopyString::new(name),
            supertypes: vec![],
            parse_id

        }
    }

    pub fn null() -> ClassMeta {
        ClassMeta {
            name: CopyString::new("null"),
            supertypes: vec![],
            parse_id: 0
        }
    }

    pub fn builder() -> ClassMetaBuilder {
        return ClassMetaBuilder::default()
    }
}

impl Display for ClassMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Default)]
pub struct ClassMetaBuilder {
    name: String,
    supertypes: Vec<ClassMeta>,
    parse_id: u32,
    methods: Vec<Method>,
    members: Vec<Arc<Strawberry<LeBlancObject>>>
}

impl ClassMetaBuilder {
    pub fn name(&mut self, name: String) -> &mut ClassMetaBuilder {
        self.name = name;
        self
    }

    pub fn supertype(&mut self, meta: ClassMeta) -> &mut ClassMetaBuilder {
        self.supertypes.push(meta);
        self
    }

    pub fn parse_id(&mut self, id: u32) -> &mut ClassMetaBuilder {
        self.parse_id = id;
        self
    }

    pub fn method(&mut self, method: Method) -> &mut ClassMetaBuilder {
        self.methods.push(method);
        self
    }

    pub fn build(&self) -> ClassMeta {
        ClassMeta::new(self.name.clone().to_cstring(), self.supertypes.clone(), self.parse_id)
    }

    pub fn build_object(&self) -> LeBlancObject {
        let mut obj = leblanc_object_custom(self.build());
        let mut methods = Arc::unwrap_or_clone(obj.methods);
        self.methods.iter().cloned().for_each(|m| {methods.insert(m);});
        obj.methods = Arc::new(methods);
        obj
    }
}