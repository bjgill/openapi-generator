/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for FakeOuterBooleanSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_BOOLEAN_SERIALIZE_OUTPUT_BOOLEAN: Mime = "*/*".parse().unwrap();
    }
    /// Create Mime objects for the response content types for FakeOuterCompositeSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_COMPOSITE_SERIALIZE_OUTPUT_COMPOSITE: Mime = "*/*".parse().unwrap();
    }
    /// Create Mime objects for the response content types for FakeOuterNumberSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_NUMBER_SERIALIZE_OUTPUT_NUMBER: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for FakeOuterStringSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_STRING_SERIALIZE_OUTPUT_STRING: Mime = "*/*".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for FakeOuterBooleanSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_BOOLEAN_SERIALIZE: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for FakeOuterCompositeSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_COMPOSITE_SERIALIZE: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for FakeOuterNumberSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_NUMBER_SERIALIZE: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for FakeOuterStringSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_STRING_SERIALIZE: Mime = "application/json".parse().unwrap();
    }

}
