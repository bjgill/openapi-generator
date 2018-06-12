//! Server implementation of simple.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use simple::{Api, ApiError,
                      FakeOuterBooleanSerializeResponse,
                      FakeOuterCompositeSerializeResponse,
                      FakeOuterNumberSerializeResponse,
                      FakeOuterStringSerializeResponse
};
use simple::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{


    fn fake_outer_boolean_serialize(&self, body: Option<bool>, context: &C) -> Box<Future<Item=FakeOuterBooleanSerializeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("fake_outer_boolean_serialize({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn fake_outer_composite_serialize(&self, outer_composite: Option<models::OuterComposite>, context: &C) -> Box<Future<Item=FakeOuterCompositeSerializeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("fake_outer_composite_serialize({:?}) - X-Span-ID: {:?}", outer_composite, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn fake_outer_number_serialize(&self, body: Option<f64>, context: &C) -> Box<Future<Item=FakeOuterNumberSerializeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("fake_outer_number_serialize({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn fake_outer_string_serialize(&self, body: Option<String>, context: &C) -> Box<Future<Item=FakeOuterStringSerializeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("fake_outer_string_serialize({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
