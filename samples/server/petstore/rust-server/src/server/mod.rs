#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;
extern crate uuid;
extern crate chrono;
extern crate multipart;
extern crate percent_encoding;
extern crate url;


use std::sync::Arc;
use std::marker::PhantomData;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use hyper::header::{Headers, ContentType};
use self::url::form_urlencoded;
use mimetypes;
use self::multipart::server::Multipart;
use self::multipart::server::save::SaveResult;

use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::{ApiError, XSpanId, XSpanIdString, Has};
use swagger::auth::Scopes;

use {Api,
     LoginUserResponse
     };
#[allow(unused_imports)]
use models;

pub mod auth;

header! { (Warning, "Warning") => [String] }

mod paths {
    extern crate regex;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(&[
            r"^/v2/user/login$"
        ]).unwrap();
    }
    pub static ID_USER_LOGIN: usize = 0;
}

pub struct NewService<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> NewService<T, C> {
        NewService{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::NewService for NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Instance = Service<T, C>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        Ok(Service::new(self.api_impl.clone()))
    }
}

pub struct Service<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> Service<T, C> {
        Service{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::Service for Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString>  + 'static
{
    type Request = (Request, C);
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item=Response, Error=Error>>;

    fn call(&self, (req, mut context): Self::Request) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (method, uri, _, headers, body) = req.deconstruct();
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());
        match &method {

            // LoginUser - GET /user/login
            &hyper::Method::Get if path.matched(paths::ID_USER_LOGIN) => {







                Box::new({
                        {{

                                Box::new(api_impl.login_user(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                LoginUserResponse::SuccessfulOperation

                                                    {
                                                        body,
                                                        x_rate_limit
                                                    }


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());
                                                    header! { (ResponseXRateLimit, "X-Rate-Limit") => [i32] }
                                                    response.headers_mut().set(ResponseXRateLimit(x_rate_limit));

                                                    response.headers_mut().set(ContentType(mimetypes::responses::LOGIN_USER_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                LoginUserResponse::Wtf

                                                    {
                                                        body,
                                                        x_rate_limit
                                                    }


                                                => {
                                                    response.set_status(StatusCode::try_from(201).unwrap());
                                                    header! { (ResponseXRateLimit, "X-Rate-Limit") => [i32] }
                                                    response.headers_mut().set(ResponseXRateLimit(x_rate_limit));

                                                    response.headers_mut().set(ContentType(mimetypes::responses::LOGIN_USER_WTF.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                LoginUserResponse::InvalidUsername


                                                    {
                                                        x_rate_limit
                                                    }

                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());
                                                    header! { (ResponseXRateLimit, "X-Rate-Limit") => [i32] }
                                                    response.headers_mut().set(ResponseXRateLimit(x_rate_limit));

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.set_status(StatusCode::InternalServerError);
                                                response.set_body("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))

                        }}
                }) as Box<Future<Item=Response, Error=Error>>


            },


            _ => Box::new(future::ok(Response::new().with_status(StatusCode::NotFound))) as Box<Future<Item=Response, Error=Error>>,
        }
    }
}

/// Utility function to get the multipart boundary marker (if any) from the Headers.
fn multipart_boundary<'a>(headers: &'a Headers) -> Option<&'a str> {
    headers.get::<ContentType>().and_then(|content_type| {
        let ContentType(ref mime) = *content_type;
        if mime.type_() == mime::MULTIPART && mime.subtype() == mime::FORM_DATA {
            mime.get_param(mime::BOUNDARY).map(|x| x.as_str())
        } else {
            None
        }
    })
}
