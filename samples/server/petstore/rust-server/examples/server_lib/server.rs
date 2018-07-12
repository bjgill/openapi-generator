//! Server implementation of petstore_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use futures::Stream;
use std::collections::HashMap;
use std::io::Error;
use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use petstore_api::{Api, ApiError,
                      LoginUserResponse
};
use petstore_api::models;

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

    /// Logs user into the system
    fn login_user(&self, context: &C) -> Box<Future<Item=LoginUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("login_user() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
