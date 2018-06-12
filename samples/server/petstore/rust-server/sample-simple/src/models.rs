#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use futures::Stream;
use models;
use swagger;


#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]

pub struct OuterComposite(object);

impl ::std::convert::From<object> for OuterComposite {
    fn from(x: object) -> Self {
        OuterComposite(x)
    }
}

impl ::std::convert::From<OuterComposite> for object {
    fn from(x: OuterComposite) -> Self {
        x.0
    }
}

impl ::std::ops::Deref for OuterComposite {
    type Target = object;
    fn deref(&self) -> &object {
        &self.0
    }
}

impl ::std::ops::DerefMut for OuterComposite {
    fn deref_mut(&mut self) -> &mut object {
        &mut self.0
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum OuterEnum { 
    #[serde(rename = "placed")]
    PLACED,
    #[serde(rename = "approved")]
    APPROVED,
    #[serde(rename = "delivered")]
    DELIVERED,
}

impl ::std::fmt::Display for OuterEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self { 
            OuterEnum::PLACED => write!(f, "{}", "placed"),
            OuterEnum::APPROVED => write!(f, "{}", "approved"),
            OuterEnum::DELIVERED => write!(f, "{}", "delivered"),
        }
    }
}

impl ::std::str::FromStr for OuterEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "placed" => Ok(OuterEnum::PLACED),
            "approved" => Ok(OuterEnum::APPROVED),
            "delivered" => Ok(OuterEnum::DELIVERED),
            _ => Err(()),
        }
    }
}
