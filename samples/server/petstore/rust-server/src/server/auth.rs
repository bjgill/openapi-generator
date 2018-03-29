use std::io;
use std::marker::PhantomData;
use std::default::Default;
use hyper;
use hyper::{Request, Response, Error, StatusCode};
use server::url::form_urlencoded;
use swagger::auth::{Authorization, AuthData, Scopes};
use swagger::context::{ExtendsWith, XSpanIdString};
use Api;

pub struct NewService<T, C, D, E>
where
    T: hyper::server::NewService<Request = (Request, E), Response = Response, Error = Error>,
    C: Default,
    D: ExtendsWith<Inner=C, Ext=XSpanIdString>,
    E: ExtendsWith<Inner=D, Ext=Option<AuthData>>,
{
    inner: T,
    marker1: PhantomData<C>,
    marker2: PhantomData<D>,
    marker3: PhantomData<E>,
}

impl<T, C, D, E> NewService<T, C, D, E>
where
    T: hyper::server::NewService<
        Request = (Request, E),
        Response = Response,
        Error = Error,
    >
        + 'static,
    C: Default,
    D: ExtendsWith<Inner=C, Ext=XSpanIdString>,
    E: ExtendsWith<Inner=D, Ext=Option<AuthData>>,
{
    pub fn new(inner: T) -> NewService<T, C, D, E> {
        NewService {
            inner,
            marker1: PhantomData,
            marker2: PhantomData,
            marker3: PhantomData,
        }
    }
}

impl<T, C, D, E> hyper::server::NewService for NewService<T, C, D, E>
    where
        T: hyper::server::NewService<Request=(Request, E), Response=Response, Error=Error> + 'static,
        C: Default,
        D: ExtendsWith<Inner=C, Ext=XSpanIdString>,
        E: ExtendsWith<Inner=D, Ext=Option<AuthData>>,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Instance = Service<T::Instance, C, D, E>;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        self.inner.new_service().map(|s| Service::new(s))
    }
}

/// Middleware to extract authentication data from request
pub struct Service<T, C, D, E>
where
    T: hyper::server::Service<Request = (Request, E), Response = Response, Error = Error>,
    C: Default,
    D: ExtendsWith<Inner=C, Ext=XSpanIdString>,
    E: ExtendsWith<Inner=D, Ext=Option<AuthData>>,
{
    inner: T,
    marker1: PhantomData<C>,
    marker2: PhantomData<D>,
    marker3: PhantomData<E>,
}

impl<T, C, D, E> Service<T, C, D, E>
where
    T: hyper::server::Service<
        Request = (Request, E),
        Response = Response,
        Error = Error,
    >,
    C: Default,
    D: ExtendsWith<Inner=C, Ext=XSpanIdString>,
    E: ExtendsWith<Inner=D, Ext=Option<AuthData>>,
{
    pub fn new(inner: T) -> Service<T, C, D, E> {
        Service {
            inner,
            marker1: PhantomData,
            marker2: PhantomData,
            marker3: PhantomData,
        }
    }
}

impl<T, C, D, E> hyper::server::Service for Service<T, C, D, E>
    where
        T: hyper::server::Service<Request=(Request, E), Response=Response, Error=Error>,
        C: Default,
        D: ExtendsWith<Inner=C, Ext=XSpanIdString>,
        E: ExtendsWith<Inner=D, Ext=Option<AuthData>>,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = T::Future;

    fn call(&self, req: Self::Request) -> Self::Future {
        let context = D::new(C::default(), XSpanIdString::get_or_generate(&req));

        {
            header! { (ApiKey1, "api_key") => [String] }
            if let Some(header) = req.headers().get::<ApiKey1>().cloned() {
                let auth_data = AuthData::ApiKey(header.0);
                let context = E::new(context, Some(auth_data));
                return self.inner.call((req, context));
            }
        }
        {
            let key = form_urlencoded::parse(req.query().unwrap_or_default().as_bytes())
                .filter(|e| e.0 == "api_key_query")
                .map(|e| e.1.clone().into_owned())
                .nth(0);
            if let Some(key) = key {
                let auth_data = AuthData::ApiKey(key);
                let context = E::new(context, Some(auth_data));
                return self.inner.call((req, context));
            }
        }
        {
            use hyper::header::{Authorization, Basic, Bearer};
            use std::ops::Deref;
            if let Some(basic) = req.headers().get::<Authorization<Basic>>().cloned() {
                let auth_data = AuthData::Basic(basic.deref().clone());
                let context = E::new(context, Some(auth_data));
                return self.inner.call((req, context));
            }
        }
        {
            use hyper::header::{Authorization, Basic, Bearer};
            use std::ops::Deref;
            if let Some(bearer) = req.headers().get::<Authorization<Bearer>>().cloned() {
                let auth_data = AuthData::Bearer(bearer.deref().clone());
                let context = E::new(context, Some(auth_data));
                return self.inner.call((req, context));
            }
        }

        let context = E::new(context, None);
        return self.inner.call((req, context));
    }
}
