#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};
use std::fmt;
use std::fmt::Debug;

pub const BASE_PATH: &'static str = "/v2";
pub const API_VERSION: &'static str = "1.0.0";


#[derive(Debug, PartialEq)]
pub enum FakeOuterBooleanSerializeResponse {
    /// Output boolean
    OutputBoolean ( models::OuterBoolean ) ,
}

#[derive(Debug, PartialEq)]
pub enum FakeOuterCompositeSerializeResponse {
    /// Output composite
    OutputComposite ( models::OuterComposite ) ,
}

#[derive(Debug, PartialEq)]
pub enum FakeOuterNumberSerializeResponse {
    /// Output number
    OutputNumber ( models::OuterNumber ) ,
}

#[derive(Debug, PartialEq)]
pub enum FakeOuterStringSerializeResponse {
    /// Output string
    OutputString ( models::OuterString ) ,
}


/// API
pub trait Api<C> {


    fn fake_outer_boolean_serialize(&self, body: Option<bool>, context: &C) -> Box<Future<Item=FakeOuterBooleanSerializeResponse, Error=ApiError>>;


    fn fake_outer_composite_serialize(&self, outer_composite: Option<models::OuterComposite>, context: &C) -> Box<Future<Item=FakeOuterCompositeSerializeResponse, Error=ApiError>>;


    fn fake_outer_number_serialize(&self, body: Option<f64>, context: &C) -> Box<Future<Item=FakeOuterNumberSerializeResponse, Error=ApiError>>;


    fn fake_outer_string_serialize(&self, body: Option<String>, context: &C) -> Box<Future<Item=FakeOuterStringSerializeResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {


    fn fake_outer_boolean_serialize(&self, body: Option<bool>) -> Box<Future<Item=FakeOuterBooleanSerializeResponse, Error=ApiError>>;


    fn fake_outer_composite_serialize(&self, outer_composite: Option<models::OuterComposite>) -> Box<Future<Item=FakeOuterCompositeSerializeResponse, Error=ApiError>>;


    fn fake_outer_number_serialize(&self, body: Option<f64>) -> Box<Future<Item=FakeOuterNumberSerializeResponse, Error=ApiError>>;


    fn fake_outer_string_serialize(&self, body: Option<String>) -> Box<Future<Item=FakeOuterStringSerializeResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {


    fn fake_outer_boolean_serialize(&self, body: Option<bool>) -> Box<Future<Item=FakeOuterBooleanSerializeResponse, Error=ApiError>> {
        self.api().fake_outer_boolean_serialize(body, &self.context())
    }


    fn fake_outer_composite_serialize(&self, outer_composite: Option<models::OuterComposite>) -> Box<Future<Item=FakeOuterCompositeSerializeResponse, Error=ApiError>> {
        self.api().fake_outer_composite_serialize(outer_composite, &self.context())
    }


    fn fake_outer_number_serialize(&self, body: Option<f64>) -> Box<Future<Item=FakeOuterNumberSerializeResponse, Error=ApiError>> {
        self.api().fake_outer_number_serialize(body, &self.context())
    }


    fn fake_outer_string_serialize(&self, body: Option<String>) -> Box<Future<Item=FakeOuterStringSerializeResponse, Error=ApiError>> {
        self.api().fake_outer_string_serialize(body, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
