#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate tokio_core;
extern crate native_tls;
extern crate hyper_tls;
extern crate openssl;
extern crate mime;

extern crate chrono;
extern crate multipart;
extern crate percent_encoding;
extern crate url;

use uuid;
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
use serde_xml_rs;

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
     TestSpecialTagsResponse,
     GetXmlFeaturesResponse,
     PostPlainTextResponse,
     PostUrlEncodedFormResponse,
     PostXmlFeaturesResponse,
     PutPlainTextResponse,
     UuidHeaderResponse,
     FakeOuterBooleanSerializeResponse,
     FakeOuterCompositeSerializeResponse,
     FakeOuterNumberSerializeResponse,
     FakeOuterStringSerializeResponse,
     TestBodyWithQueryParamsResponse,
     TestClientModelResponse,
     TestEndpointParametersResponse,
     TestEnumParametersResponse,
     TestJsonFormDataResponse,
     TestClassnameResponse,
     AddPetResponse,
     DeletePetResponse,
     FindPetsByStatusResponse,
     FindPetsByTagsResponse,
     GetPetByIdResponse,
     UpdatePetResponse,
     UpdatePetWithFormResponse,
     UploadFileResponse,
     DeleteOrderResponse,
     GetInventoryResponse,
     GetOrderByIdResponse,
     GetStoreFileResponse,
     PlaceOrderResponse,
     CreateUserResponse,
     CreateUsersWithArrayInputResponse,
     CreateUsersWithListInputResponse,
     DeleteUserResponse,
     GetUserByNameResponse,
     LoginUserResponse,
     LogoutUserResponse,
     UpdateUserResponse
     };
#[allow(unused_imports)]
use models;

pub mod auth;

header! { (Warning, "Warning") => [String] }

mod paths {
    extern crate regex;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(&[
            r"^/v2/another-fake/dummy$",
            r"^/v2/fake$",
            r"^/v2/fake/body-with-query-params$",
            r"^/v2/fake/formUrlEncoded$",
            r"^/v2/fake/jsonFormData$",
            r"^/v2/fake/outer/boolean$",
            r"^/v2/fake/outer/composite$",
            r"^/v2/fake/outer/number$",
            r"^/v2/fake/outer/string$",
            r"^/v2/fake/plaintext$",
            r"^/v2/fake/xmlFeatures$",
            r"^/v2/fake_classname_test$",
            r"^/v2/pet$",
            r"^/v2/pet/findByStatus$",
            r"^/v2/pet/findByTags$",
            r"^/v2/pet/(?P<petId>[^/?#]*)$",
            r"^/v2/pet/(?P<petId>[^/?#]*)/uploadImage$",
            r"^/v2/store$",
            r"^/v2/store/inventory$",
            r"^/v2/store/order$",
            r"^/v2/store/order/(?P<order_id>[^/?#]*)$",
            r"^/v2/user$",
            r"^/v2/user/createWithArray$",
            r"^/v2/user/createWithList$",
            r"^/v2/user/login$",
            r"^/v2/user/logout$",
            r"^/v2/user/(?P<username>[^/?#]*)$",
            r"^/v2/uuid-headers/$"
        ]).unwrap();
    }
    pub static ID_ANOTHER_FAKE_DUMMY: usize = 0;
    pub static ID_FAKE: usize = 1;
    pub static ID_FAKE_BODY_WITH_QUERY_PARAMS: usize = 2;
    pub static ID_FAKE_FORMURLENCODED: usize = 3;
    pub static ID_FAKE_JSONFORMDATA: usize = 4;
    pub static ID_FAKE_OUTER_BOOLEAN: usize = 5;
    pub static ID_FAKE_OUTER_COMPOSITE: usize = 6;
    pub static ID_FAKE_OUTER_NUMBER: usize = 7;
    pub static ID_FAKE_OUTER_STRING: usize = 8;
    pub static ID_FAKE_PLAINTEXT: usize = 9;
    pub static ID_FAKE_XMLFEATURES: usize = 10;
    pub static ID_FAKE_CLASSNAME_TEST: usize = 11;
    pub static ID_PET: usize = 12;
    pub static ID_PET_FINDBYSTATUS: usize = 13;
    pub static ID_PET_FINDBYTAGS: usize = 14;
    pub static ID_PET_PETID: usize = 15;
    lazy_static! {
        pub static ref REGEX_PET_PETID: regex::Regex = regex::Regex::new(r"^/v2/pet/(?P<petId>[^/?#]*)$").unwrap();
    }
    pub static ID_PET_PETID_UPLOADIMAGE: usize = 16;
    lazy_static! {
        pub static ref REGEX_PET_PETID_UPLOADIMAGE: regex::Regex = regex::Regex::new(r"^/v2/pet/(?P<petId>[^/?#]*)/uploadImage$").unwrap();
    }
    pub static ID_STORE: usize = 17;
    pub static ID_STORE_INVENTORY: usize = 18;
    pub static ID_STORE_ORDER: usize = 19;
    pub static ID_STORE_ORDER_ORDER_ID: usize = 20;
    lazy_static! {
        pub static ref REGEX_STORE_ORDER_ORDER_ID: regex::Regex = regex::Regex::new(r"^/v2/store/order/(?P<order_id>[^/?#]*)$").unwrap();
    }
    pub static ID_USER: usize = 21;
    pub static ID_USER_CREATEWITHARRAY: usize = 22;
    pub static ID_USER_CREATEWITHLIST: usize = 23;
    pub static ID_USER_LOGIN: usize = 24;
    pub static ID_USER_LOGOUT: usize = 25;
    pub static ID_USER_USERNAME: usize = 26;
    lazy_static! {
        pub static ref REGEX_USER_USERNAME: regex::Regex = regex::Regex::new(r"^/v2/user/(?P<username>[^/?#]*)$").unwrap();
    }
    pub static ID_UUID_HEADERS_: usize = 27;
}

pub struct NewService<T, C> {
    api_impl: Arc<T>,
    marker: PhantomData<C>,
}

impl<T, C> NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
{
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> NewService<T, C> {
        NewService{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::NewService for NewService<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
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
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static {
    pub fn new<U: Into<Arc<T>>>(api_impl: U) -> Service<T, C> {
        Service{api_impl: api_impl.into(), marker: PhantomData}
    }
}

impl<T, C> hyper::server::Service for Service<T, C>
where
    T: Api<C> + Clone + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + 'static
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

            // TestSpecialTags - PATCH /another-fake/dummy
            &hyper::Method::Patch if path.matched(paths::ID_ANOTHER_FAKE_DUMMY) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_client: Option<Client> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_client) => param_client,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter Client - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_client = match param_client {
                                    Some(param_client) => param_client,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter Client"))),
                                };


                                Box::new(api_impl.test_special_tags(param_client, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TestSpecialTagsResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::TEST_SPECIAL_TAGS_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter Client: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // GetXmlFeatures - GET /fake/xmlFeatures
            &hyper::Method::Get if path.matched(paths::ID_FAKE_XMLFEATURES) => {







                Box::new({
                        {{

                                Box::new(api_impl.get_xml_features(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetXmlFeaturesResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_XML_FEATURES_SUCCESS.clone()));


                                                    let mut namespaces = BTreeMap::new();
                                                    // An empty string is used to indicate a global namespace in xmltree.
                                                    namespaces.insert("".to_string(), models::namespaces::XMLOBJECT.clone());
                                                    let body = serde_xml_rs::to_string_with_namespaces(&body, namespaces).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


            // PostPlainText - POST /fake/plaintext
            &hyper::Method::Post if path.matched(paths::ID_FAKE_PLAINTEXT) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {
                                let param_body: Option<String> = if !body.is_empty() {

                                    match String::from_utf8(body.to_vec()) {
                                        Ok(param_body) => Some(param_body),
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter body - not valid UTF-8: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_body = match param_body {
                                    Some(param_body) => param_body,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter body"))),
                                };


                                Box::new(api_impl.post_plain_text(param_body, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostPlainTextResponse::Success

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::POST_PLAIN_TEXT_SUCCESS.clone()));


                                                    response.set_body(body);
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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter body: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // PostUrlEncodedForm - POST /fake/formUrlEncoded
            &hyper::Method::Post if path.matched(paths::ID_FAKE_FORMURLENCODED) => {







                Box::new({
                        {{

                                // Form parameters
                                let param_param1 = "param1_example".to_string();
                                let param_param2 = "param2_example".to_string();
                                let param_param3 = Some("param3_example".to_string());

                                Box::new(api_impl.post_url_encoded_form(param_param1, param_param2, param_param3, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostUrlEncodedFormResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

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


            // PostXmlFeatures - POST /fake/xmlFeatures
            &hyper::Method::Post if path.matched(paths::ID_FAKE_XMLFEATURES) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_xml_object: Option<XmlObject> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_xml_object) => param_xml_object,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter XmlObject - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_xml_object = match param_xml_object {
                                    Some(param_xml_object) => param_xml_object,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter XmlObject"))),
                                };


                                Box::new(api_impl.post_xml_features(param_xml_object, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PostXmlFeaturesResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter XmlObject: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // PutPlainText - PUT /fake/plaintext
            &hyper::Method::Put if path.matched(paths::ID_FAKE_PLAINTEXT) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {
                                let param_body: Option<String> = if !body.is_empty() {

                                    match String::from_utf8(body.to_vec()) {
                                        Ok(param_body) => Some(param_body),
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter body - not valid UTF-8: {}", e)))),
                                    }

                                } else {
                                    None
                                };


                                Box::new(api_impl.put_plain_text(param_body, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PutPlainTextResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter body: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // UuidHeader - POST /uuid-headers/
            &hyper::Method::Post if path.matched(paths::ID_UUID_HEADERS_) => {



                // Header parameters
                header! { (RequestXUuidHeader, "X-Uuid-Header") => [uuid::Uuid] }
                let param_x_uuid_header = match headers.get::<RequestXUuidHeader>() {
                    Some(param_x_uuid_header) => param_x_uuid_header.0.clone(),
                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing or invalid required header X-Uuid-Header"))),
                };





                Box::new({
                        {{

                                Box::new(api_impl.uuid_header(param_x_uuid_header, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UuidHeaderResponse::SuccessOrNotFound


                                                    {
                                                        x_uuid_header
                                                    }

                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());
                                                    header! { (ResponseXUuidHeader, "X-Uuid-Header") => [uuid::Uuid] }
                                                    response.headers_mut().set(ResponseXUuidHeader(x_uuid_header));

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


            // FakeOuterBooleanSerialize - POST /fake/outer/boolean
            &hyper::Method::Post if path.matched(paths::ID_FAKE_OUTER_BOOLEAN) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_body: Option<bool> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,

                                        Err(_) => None,
                                    }

                                } else {
                                    None
                                };


                                Box::new(api_impl.fake_outer_boolean_serialize(param_body, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                FakeOuterBooleanSerializeResponse::OutputBoolean

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::FAKE_OUTER_BOOLEAN_SERIALIZE_OUTPUT_BOOLEAN.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter body: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // FakeOuterCompositeSerialize - POST /fake/outer/composite
            &hyper::Method::Post if path.matched(paths::ID_FAKE_OUTER_COMPOSITE) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_outer_composite: Option<OuterComposite> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_outer_composite) => param_outer_composite,

                                        Err(_) => None,
                                    }

                                } else {
                                    None
                                };


                                Box::new(api_impl.fake_outer_composite_serialize(param_outer_composite, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                FakeOuterCompositeSerializeResponse::OutputComposite

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::FAKE_OUTER_COMPOSITE_SERIALIZE_OUTPUT_COMPOSITE.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter OuterComposite: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // FakeOuterNumberSerialize - POST /fake/outer/number
            &hyper::Method::Post if path.matched(paths::ID_FAKE_OUTER_NUMBER) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_body: Option<f64> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,

                                        Err(_) => None,
                                    }

                                } else {
                                    None
                                };


                                Box::new(api_impl.fake_outer_number_serialize(param_body, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                FakeOuterNumberSerializeResponse::OutputNumber

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::FAKE_OUTER_NUMBER_SERIALIZE_OUTPUT_NUMBER.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter body: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // FakeOuterStringSerialize - POST /fake/outer/string
            &hyper::Method::Post if path.matched(paths::ID_FAKE_OUTER_STRING) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_body: Option<String> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_body) => param_body,

                                        Err(_) => None,
                                    }

                                } else {
                                    None
                                };


                                Box::new(api_impl.fake_outer_string_serialize(param_body, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                FakeOuterStringSerializeResponse::OutputString

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::FAKE_OUTER_STRING_SERIALIZE_OUTPUT_STRING.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter body: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // TestBodyWithQueryParams - PUT /fake/body-with-query-params
            &hyper::Method::Put if path.matched(paths::ID_FAKE_BODY_WITH_QUERY_PARAMS) => {





                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_query = query_params.iter().filter(|e| e.0 == "query").map(|e| e.1.to_owned())

                    .nth(0);
                let param_query = match param_query {
                    Some(param_query) => match param_query.parse::<String>() {
                        Ok(param_query) => param_query,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse query parameter query - doesn't match schema: {}", e)))),
                    },
                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required query parameter query"))),
                };


                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_user: Option<User> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user) => param_user,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter User - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_user = match param_user {
                                    Some(param_user) => param_user,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter User"))),
                                };


                                Box::new(api_impl.test_body_with_query_params(param_query, param_user, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TestBodyWithQueryParamsResponse::Success


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter User: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // TestClientModel - PATCH /fake
            &hyper::Method::Patch if path.matched(paths::ID_FAKE) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_client: Option<Client> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_client) => param_client,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter Client - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_client = match param_client {
                                    Some(param_client) => param_client,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter Client"))),
                                };


                                Box::new(api_impl.test_client_model(param_client, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TestClientModelResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::TEST_CLIENT_MODEL_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter Client: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // TestEndpointParameters - POST /fake
            &hyper::Method::Post if path.matched(paths::ID_FAKE) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_unknown_base_type: Option<models::object> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_unknown_base_type) => param_unknown_base_type,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter UNKNOWN_BASE_TYPE - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_unknown_base_type = match param_unknown_base_type {
                                    Some(param_unknown_base_type) => param_unknown_base_type,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter UNKNOWN_BASE_TYPE"))),
                                };


                                Box::new(api_impl.test_endpoint_parameters(param_unknown_base_type, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TestEndpointParametersResponse::InvalidUsernameSupplied


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                TestEndpointParametersResponse::UserNotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter UNKNOWN_BASE_TYPE: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // TestEnumParameters - GET /fake
            &hyper::Method::Get if path.matched(paths::ID_FAKE) => {



                // Header parameters
                header! { (RequestEnumHeaderStringArray, "enum_header_string_array") => (String)* }
                let param_enum_header_string_array = headers.get::<RequestEnumHeaderStringArray>().map(|header| header.0.clone());
                header! { (RequestEnumHeaderString, "enum_header_string") => [String] }
                let param_enum_header_string = headers.get::<RequestEnumHeaderString>().map(|header| header.0.clone());



                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_enum_query_string_array = query_params.iter().filter(|e| e.0 == "enum_query_string_array").map(|e| e.1.to_owned())
                    .filter_map(|param_enum_query_string_array| param_enum_query_string_array.parse::<String>().ok())
                    .collect::<Vec<_>>();
                let param_enum_query_string_array = if !param_enum_query_string_array.is_empty() {
                    Some(param_enum_query_string_array)
                } else {
                    None
                };
                let param_enum_query_string = query_params.iter().filter(|e| e.0 == "enum_query_string").map(|e| e.1.to_owned())

                    .nth(0);

                let param_enum_query_string = param_enum_query_string.and_then(|param_enum_query_string| param_enum_query_string.parse::<>().ok());
                let param_enum_query_integer = query_params.iter().filter(|e| e.0 == "enum_query_integer").map(|e| e.1.to_owned())

                    .nth(0);

                let param_enum_query_integer = param_enum_query_integer.and_then(|param_enum_query_integer| param_enum_query_integer.parse::<>().ok());


                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_unknown_base_type: Option<models::object> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_unknown_base_type) => param_unknown_base_type,

                                        Err(_) => None,
                                    }

                                } else {
                                    None
                                };


                                Box::new(api_impl.test_enum_parameters(param_enum_header_string_array.as_ref(), param_enum_header_string, param_enum_query_string_array.as_ref(), param_enum_query_string, param_enum_query_integer, param_unknown_base_type, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TestEnumParametersResponse::InvalidRequest


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                TestEnumParametersResponse::NotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter UNKNOWN_BASE_TYPE: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // TestJsonFormData - GET /fake/jsonFormData
            &hyper::Method::Get if path.matched(paths::ID_FAKE_JSONFORMDATA) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_unknown_base_type: Option<models::object> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_unknown_base_type) => param_unknown_base_type,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter UNKNOWN_BASE_TYPE - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_unknown_base_type = match param_unknown_base_type {
                                    Some(param_unknown_base_type) => param_unknown_base_type,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter UNKNOWN_BASE_TYPE"))),
                                };


                                Box::new(api_impl.test_json_form_data(param_unknown_base_type, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TestJsonFormDataResponse::SuccessfulOperation


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter UNKNOWN_BASE_TYPE: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // TestClassname - PATCH /fake_classname_test
            &hyper::Method::Patch if path.matched(paths::ID_FAKE_CLASSNAME_TEST) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_client: Option<Client> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_client) => param_client,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter Client - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_client = match param_client {
                                    Some(param_client) => param_client,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter Client"))),
                                };


                                Box::new(api_impl.test_classname(param_client, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TestClassnameResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::TEST_CLASSNAME_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter Client: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // AddPet - POST /pet
            &hyper::Method::Post if path.matched(paths::ID_PET) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "write:pets".to_string(), // modify pets in your account
                            "read:pets".to_string(), // read your pets
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_pet: Option<Pet> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_pet) => param_pet,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter Pet - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_pet = match param_pet {
                                    Some(param_pet) => param_pet,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter Pet"))),
                                };


                                Box::new(api_impl.add_pet(param_pet, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddPetResponse::InvalidInput


                                                => {
                                                    response.set_status(StatusCode::try_from(405).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter Pet: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // DeletePet - DELETE /pet/{petId}
            &hyper::Method::Delete if path.matched(paths::ID_PET_PETID) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "write:pets".to_string(), // modify pets in your account
                            "read:pets".to_string(), // read your pets
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PET_PETID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PET_PETID in set but failed match against \"{}\"", path, paths::REGEX_PET_PETID.as_str())
                    );

                let param_pet_id = match percent_encoding::percent_decode(path_params["petId"].as_bytes()).decode_utf8() {
                    Ok(param_pet_id) => match param_pet_id.parse::<i64>() {
                        Ok(param_pet_id) => param_pet_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter petId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["petId"]))))
                };

                // Header parameters
                header! { (RequestApiKey, "api_key") => [String] }
                let param_api_key = headers.get::<RequestApiKey>().map(|header| header.0.clone());





                Box::new({
                        {{

                                Box::new(api_impl.delete_pet(param_pet_id, param_api_key, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeletePetResponse::InvalidPetValue


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

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


            // FindPetsByStatus - GET /pet/findByStatus
            &hyper::Method::Get if path.matched(paths::ID_PET_FINDBYSTATUS) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "write:pets".to_string(), // modify pets in your account
                            "read:pets".to_string(), // read your pets
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }





                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_status = query_params.iter().filter(|e| e.0 == "status").map(|e| e.1.to_owned())
                    .filter_map(|param_status| param_status.parse::<String>().ok())
                    .collect::<Vec<_>>();



                Box::new({
                        {{

                                Box::new(api_impl.find_pets_by_status(param_status.as_ref(), &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                FindPetsByStatusResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::FIND_PETS_BY_STATUS_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                FindPetsByStatusResponse::InvalidStatusValue


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

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


            // FindPetsByTags - GET /pet/findByTags
            &hyper::Method::Get if path.matched(paths::ID_PET_FINDBYTAGS) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "write:pets".to_string(), // modify pets in your account
                            "read:pets".to_string(), // read your pets
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }





                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_tags = query_params.iter().filter(|e| e.0 == "tags").map(|e| e.1.to_owned())
                    .filter_map(|param_tags| param_tags.parse::<String>().ok())
                    .collect::<Vec<_>>();



                Box::new({
                        {{

                                Box::new(api_impl.find_pets_by_tags(param_tags.as_ref(), &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                FindPetsByTagsResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::FIND_PETS_BY_TAGS_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                FindPetsByTagsResponse::InvalidTagValue


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

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


            // GetPetById - GET /pet/{petId}
            &hyper::Method::Get if path.matched(paths::ID_PET_PETID) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PET_PETID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PET_PETID in set but failed match against \"{}\"", path, paths::REGEX_PET_PETID.as_str())
                    );

                let param_pet_id = match percent_encoding::percent_decode(path_params["petId"].as_bytes()).decode_utf8() {
                    Ok(param_pet_id) => match param_pet_id.parse::<i64>() {
                        Ok(param_pet_id) => param_pet_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter petId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["petId"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_pet_by_id(param_pet_id, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetPetByIdResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_PET_BY_ID_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetPetByIdResponse::InvalidIDSupplied


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                GetPetByIdResponse::PetNotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

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


            // UpdatePet - PUT /pet
            &hyper::Method::Put if path.matched(paths::ID_PET) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "write:pets".to_string(), // modify pets in your account
                            "read:pets".to_string(), // read your pets
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_pet: Option<Pet> = if !body.is_empty() {
                                    let deserializer = &mut serde_xml_rs::de::Deserializer::new_from_reader(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_pet) => param_pet,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter Pet - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_pet = match param_pet {
                                    Some(param_pet) => param_pet,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter Pet"))),
                                };


                                Box::new(api_impl.update_pet(param_pet, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdatePetResponse::InvalidIDSupplied


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                UpdatePetResponse::PetNotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

                                                },
                                                UpdatePetResponse::ValidationException


                                                => {
                                                    response.set_status(StatusCode::try_from(405).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter Pet: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // UpdatePetWithForm - POST /pet/{petId}
            &hyper::Method::Post if path.matched(paths::ID_PET_PETID) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "write:pets".to_string(), // modify pets in your account
                            "read:pets".to_string(), // read your pets
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PET_PETID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PET_PETID in set but failed match against \"{}\"", path, paths::REGEX_PET_PETID.as_str())
                    );

                let param_pet_id = match percent_encoding::percent_decode(path_params["petId"].as_bytes()).decode_utf8() {
                    Ok(param_pet_id) => match param_pet_id.parse::<i64>() {
                        Ok(param_pet_id) => param_pet_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter petId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["petId"]))))
                };





                Box::new({
                        {{

                                // Form parameters
                                let param_name = Some("name_example".to_string());
                                let param_status = Some("status_example".to_string());

                                Box::new(api_impl.update_pet_with_form(param_pet_id, param_name, param_status, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdatePetWithFormResponse::InvalidInput


                                                => {
                                                    response.set_status(StatusCode::try_from(405).unwrap());

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


            // UploadFile - POST /pet/{petId}/uploadImage
            &hyper::Method::Post if path.matched(paths::ID_PET_PETID_UPLOADIMAGE) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: BTreeSet<String> = vec![
                            "write:pets".to_string(), // modify pets in your account
                            "read:pets".to_string(), // read your pets
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Box::new(future::ok(Response::new()
                                .with_status(StatusCode::Forbidden)
                                .with_body(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope)
                                ))
                            ));
                        }
                    }
                }


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_PET_PETID_UPLOADIMAGE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PET_PETID_UPLOADIMAGE in set but failed match against \"{}\"", path, paths::REGEX_PET_PETID_UPLOADIMAGE.as_str())
                    );

                let param_pet_id = match percent_encoding::percent_decode(path_params["petId"].as_bytes()).decode_utf8() {
                    Ok(param_pet_id) => match param_pet_id.parse::<i64>() {
                        Ok(param_pet_id) => param_pet_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter petId: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["petId"]))))
                };





                let boundary = match multipart_boundary(&headers) {
                    Some(boundary) => boundary.to_string(),
                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Couldn't find valid multipart body"))),
                };

                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {
                                let mut entries = match Multipart::with_body(&body.to_vec()[..], boundary).save().temp() {
                                    SaveResult::Full(entries) => {
                                        entries
                                    },
                                    _ => {
                                        return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Unable to process all message parts"))))
                                    },
                                };

                                // Form parameters
                                let param_additional_metadata = entries.fields.remove("additional_metadata");
                                let param_additional_metadata = match param_additional_metadata {
                                    Some(entry) =>

                                        match entry.parse::<String>() {
                                            Ok(entry) => Some(entry),

                                            Err(_) => None,
                                        },

                                    None => None,
                                };

                                let param_file = entries.fields.remove("file");
                                let param_file = match param_file {
                                    Some(entry) =>
                                        Some(Box::new(stream::once(Ok(entry.as_bytes().to_vec()))) as Box<Stream<Item=Vec<u8>, Error=io::Error> + Send>),

                                    None => None,
                                };
                                let param_file = Box::new(future::ok(param_file));

                                Box::new(api_impl.upload_file(param_pet_id, param_additional_metadata, param_file, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UploadFileResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::UPLOAD_FILE_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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

                                as Box<Future<Item=Response, Error=Error>>
                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read multipart body")))),
                        }
                    })
                )


            },


            // DeleteOrder - DELETE /store/order/{order_id}
            &hyper::Method::Delete if path.matched(paths::ID_STORE_ORDER_ORDER_ID) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_STORE_ORDER_ORDER_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STORE_ORDER_ORDER_ID in set but failed match against \"{}\"", path, paths::REGEX_STORE_ORDER_ORDER_ID.as_str())
                    );

                let param_order_id = match percent_encoding::percent_decode(path_params["order_id"].as_bytes()).decode_utf8() {
                    Ok(param_order_id) => match param_order_id.parse::<String>() {
                        Ok(param_order_id) => param_order_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter order_id: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["order_id"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.delete_order(param_order_id, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteOrderResponse::InvalidIDSupplied


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                DeleteOrderResponse::OrderNotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

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


            // GetInventory - GET /store/inventory
            &hyper::Method::Get if path.matched(paths::ID_STORE_INVENTORY) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }







                Box::new({
                        {{

                                Box::new(api_impl.get_inventory(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetInventoryResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_INVENTORY_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
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


            // GetOrderById - GET /store/order/{order_id}
            &hyper::Method::Get if path.matched(paths::ID_STORE_ORDER_ORDER_ID) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_STORE_ORDER_ORDER_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STORE_ORDER_ORDER_ID in set but failed match against \"{}\"", path, paths::REGEX_STORE_ORDER_ORDER_ID.as_str())
                    );

                let param_order_id = match percent_encoding::percent_decode(path_params["order_id"].as_bytes()).decode_utf8() {
                    Ok(param_order_id) => match param_order_id.parse::<i64>() {
                        Ok(param_order_id) => param_order_id,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter order_id: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["order_id"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_order_by_id(param_order_id, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOrderByIdResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_ORDER_BY_ID_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetOrderByIdResponse::InvalidIDSupplied


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                GetOrderByIdResponse::OrderNotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

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


            // GetStoreFile - GET /store
            &hyper::Method::Get if path.matched(paths::ID_STORE) => {
                {
                    let authorization = match (&context as &Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Box::new(future::ok(Response::new()
                                                .with_status(StatusCode::Forbidden)
                                                .with_body("Unauthenticated"))),
                    };

                }







                Box::new({
                        {{

                                Box::new(api_impl.get_store_file(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetStoreFileResponse::SuccessfulOperation

                                                    (body)


                                                => {

                                                    let body = body.fold(Vec::new(), | mut body, chunk| {
                                                        body.extend(chunk.iter());
                                                        future::ok::<Vec<u8>, io::Error>(body)
                                                    })
                                                    // Block whilst waiting for the stream to complete
                                                    .wait();

                                                    match body {
                                                        // If no error occurred then create successful hyper response
                                                        Ok(vec) => {
                                                            response.set_status(StatusCode::try_from(200).unwrap());


                                                            response.headers_mut().set(ContentType(mimetypes::responses::GET_STORE_FILE_SUCCESSFUL_OPERATION.clone()));

                                                            response.set_body(vec);
                                                        },
                                                        // It's possible that errors were received in the stream, if this is the case then we can't return a success response to the client and instead must return an internal error.
                                                        Err(e) => {
                                                            response.set_status(StatusCode::InternalServerError);
                                                            response.set_body("An internal error occurred");
                                                        }
                                                    }
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


            // PlaceOrder - POST /store/order
            &hyper::Method::Post if path.matched(paths::ID_STORE_ORDER) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_order: Option<Order> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_order) => param_order,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter Order - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_order = match param_order {
                                    Some(param_order) => param_order,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter Order"))),
                                };


                                Box::new(api_impl.place_order(param_order, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                PlaceOrderResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::PLACE_ORDER_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                PlaceOrderResponse::InvalidOrder


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter Order: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // CreateUser - POST /user
            &hyper::Method::Post if path.matched(paths::ID_USER) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_user: Option<User> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user) => param_user,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter User - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_user = match param_user {
                                    Some(param_user) => param_user,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter User"))),
                                };


                                Box::new(api_impl.create_user(param_user, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateUserResponse::SuccessfulOperation


                                                => {
                                                    response.set_status(StatusCode::try_from(0).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter User: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // CreateUsersWithArrayInput - POST /user/createWithArray
            &hyper::Method::Post if path.matched(paths::ID_USER_CREATEWITHARRAY) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_user: Option<Vec<models::User>> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user) => param_user,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter User - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_user = match param_user {
                                    Some(param_user) => param_user,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter User"))),
                                };


                                Box::new(api_impl.create_users_with_array_input(param_user.as_ref(), &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateUsersWithArrayInputResponse::SuccessfulOperation


                                                => {
                                                    response.set_status(StatusCode::try_from(0).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter User: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // CreateUsersWithListInput - POST /user/createWithList
            &hyper::Method::Post if path.matched(paths::ID_USER_CREATEWITHLIST) => {






                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_user: Option<Vec<models::User>> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user) => param_user,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter User - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_user = match param_user {
                                    Some(param_user) => param_user,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter User"))),
                                };


                                Box::new(api_impl.create_users_with_list_input(param_user.as_ref(), &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateUsersWithListInputResponse::SuccessfulOperation


                                                => {
                                                    response.set_status(StatusCode::try_from(0).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter User: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

            },


            // DeleteUser - DELETE /user/{username}
            &hyper::Method::Delete if path.matched(paths::ID_USER_USERNAME) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_USER_USERNAME
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USER_USERNAME in set but failed match against \"{}\"", path, paths::REGEX_USER_USERNAME.as_str())
                    );

                let param_username = match percent_encoding::percent_decode(path_params["username"].as_bytes()).decode_utf8() {
                    Ok(param_username) => match param_username.parse::<String>() {
                        Ok(param_username) => param_username,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter username: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["username"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.delete_user(param_username, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteUserResponse::InvalidUsernameSupplied


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                DeleteUserResponse::UserNotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

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


            // GetUserByName - GET /user/{username}
            &hyper::Method::Get if path.matched(paths::ID_USER_USERNAME) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_USER_USERNAME
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USER_USERNAME in set but failed match against \"{}\"", path, paths::REGEX_USER_USERNAME.as_str())
                    );

                let param_username = match percent_encoding::percent_decode(path_params["username"].as_bytes()).decode_utf8() {
                    Ok(param_username) => match param_username.parse::<String>() {
                        Ok(param_username) => param_username,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter username: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["username"]))))
                };





                Box::new({
                        {{

                                Box::new(api_impl.get_user_by_name(param_username, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetUserByNameResponse::SuccessfulOperation

                                                    (body)


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());

                                                    response.headers_mut().set(ContentType(mimetypes::responses::GET_USER_BY_NAME_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                GetUserByNameResponse::InvalidUsernameSupplied


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                GetUserByNameResponse::UserNotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

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


            // LoginUser - GET /user/login
            &hyper::Method::Get if path.matched(paths::ID_USER_LOGIN) => {





                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_username = query_params.iter().filter(|e| e.0 == "username").map(|e| e.1.to_owned())

                    .nth(0);
                let param_username = match param_username {
                    Some(param_username) => match param_username.parse::<String>() {
                        Ok(param_username) => param_username,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse query parameter username - doesn't match schema: {}", e)))),
                    },
                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required query parameter username"))),
                };
                let param_password = query_params.iter().filter(|e| e.0 == "password").map(|e| e.1.to_owned())

                    .nth(0);
                let param_password = match param_password {
                    Some(param_password) => match param_password.parse::<String>() {
                        Ok(param_password) => param_password,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse query parameter password - doesn't match schema: {}", e)))),
                    },
                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required query parameter password"))),
                };



                Box::new({
                        {{

                                Box::new(api_impl.login_user(param_username, param_password, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                LoginUserResponse::SuccessfulOperation

                                                    {
                                                        body,
                                                        x_rate_limit, 

                                                        x_expires_after
                                                    }


                                                => {
                                                    response.set_status(StatusCode::try_from(200).unwrap());
                                                    header! { (ResponseXRateLimit, "X-Rate-Limit") => [i32] }
                                                    response.headers_mut().set(ResponseXRateLimit(x_rate_limit));
                                                    header! { (ResponseXExpiresAfter, "X-Expires-After") => [chrono::DateTime<chrono::Utc>] }
                                                    response.headers_mut().set(ResponseXExpiresAfter(x_expires_after));

                                                    response.headers_mut().set(ContentType(mimetypes::responses::LOGIN_USER_SUCCESSFUL_OPERATION.clone()));


                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");

                                                    response.set_body(body);
                                                },
                                                LoginUserResponse::InvalidUsername


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

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


            // LogoutUser - GET /user/logout
            &hyper::Method::Get if path.matched(paths::ID_USER_LOGOUT) => {







                Box::new({
                        {{

                                Box::new(api_impl.logout_user(&context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                LogoutUserResponse::SuccessfulOperation


                                                => {
                                                    response.set_status(StatusCode::try_from(0).unwrap());

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


            // UpdateUser - PUT /user/{username}
            &hyper::Method::Put if path.matched(paths::ID_USER_USERNAME) => {


                // Path parameters
                let path = uri.path().to_string();
                let path_params =
                    paths::REGEX_USER_USERNAME
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE USER_USERNAME in set but failed match against \"{}\"", path, paths::REGEX_USER_USERNAME.as_str())
                    );

                let param_username = match percent_encoding::percent_decode(path_params["username"].as_bytes()).decode_utf8() {
                    Ok(param_username) => match param_username.parse::<String>() {
                        Ok(param_username) => param_username,
                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse path parameter username: {}", e)))),
                    },
                    Err(_) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["username"]))))
                };




                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                Box::new(body.concat2()
                    .then(move |result| -> Box<Future<Item=Response, Error=Error>> {
                        match result {
                            Ok(body) => {

                                let mut unused_elements = Vec::new();
                                let param_user: Option<User> = if !body.is_empty() {

                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);

                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_user) => param_user,
                                        Err(e) => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't parse body parameter User - doesn't match schema: {}", e)))),
                                    }

                                } else {
                                    None
                                };
                                let param_user = match param_user {
                                    Some(param_user) => param_user,
                                    None => return Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body("Missing required body parameter User"))),
                                };


                                Box::new(api_impl.update_user(param_username, param_user, &context)
                                    .then(move |result| {
                                        let mut response = Response::new();
                                        response.headers_mut().set(XSpanId((&context as &Has<XSpanIdString>).get().0.to_string()));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateUserResponse::InvalidUserSupplied


                                                => {
                                                    response.set_status(StatusCode::try_from(400).unwrap());

                                                },
                                                UpdateUserResponse::UserNotFound


                                                => {
                                                    response.set_status(StatusCode::try_from(404).unwrap());

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


                            },
                            Err(e) => Box::new(future::ok(Response::new().with_status(StatusCode::BadRequest).with_body(format!("Couldn't read body parameter User: {}", e)))),
                        }
                    })
                ) as Box<Future<Item=Response, Error=Error>>

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
