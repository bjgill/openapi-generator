#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate simple;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
#[macro_use]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;
extern crate clap;
extern crate tokio_core;

use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Has, Push, AuthData};

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
use tokio_core::reactor;
#[allow(unused_imports)]
use simple::{ApiNoContext, ContextWrapperExt,
                      ApiError,
                      FakeOuterBooleanSerializeResponse,
                      FakeOuterCompositeSerializeResponse,
                      FakeOuterNumberSerializeResponse,
                      FakeOuterStringSerializeResponse
                     };
use clap::{App, Arg};

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
    "FakeOuterBooleanSerialize",
    "FakeOuterCompositeSerialize",
    "FakeOuterNumberSerialize",
    "FakeOuterStringSerialize",
])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("petstore.swagger.io")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("80")
            .help("Port to contact"))
        .get_matches();

    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        simple::Client::try_new_https(core.handle(), &base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        simple::Client::try_new_http(core.handle(), &base_url)
            .expect("Failed to create HTTP client")
    };

    let context: make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
        make_context!(ContextBuilder, EmptyContext, None, XSpanIdString(self::uuid::Uuid::new_v4().to_string()));
    let client = client.with_context(context);

    match matches.value_of("operation") {

        Some("FakeOuterBooleanSerialize") => {
            let result = core.run(client.fake_outer_boolean_serialize(Some(true)));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("FakeOuterCompositeSerialize") => {
            let result = core.run(client.fake_outer_composite_serialize(None));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("FakeOuterNumberSerialize") => {
            let result = core.run(client.fake_outer_number_serialize(Some(1.2)));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("FakeOuterStringSerialize") => {
            let result = core.run(client.fake_outer_string_serialize(Some("body_example".to_string())));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

