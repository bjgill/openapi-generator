/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for TestSpecialTags
    lazy_static! {
        pub static ref TEST_SPECIAL_TAGS_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetXmlFeatures
    lazy_static! {
        pub static ref GET_XML_FEATURES_SUCCESS: Mime = "application/xml".parse().unwrap();
    }
    /// Create Mime objects for the response content types for PostPlainText
    lazy_static! {
        pub static ref POST_PLAIN_TEXT_SUCCESS: Mime = "text/plain".parse().unwrap();
    }
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
        pub static ref FAKE_OUTER_NUMBER_SERIALIZE_OUTPUT_NUMBER: Mime = "*/*".parse().unwrap();
    }
    /// Create Mime objects for the response content types for FakeOuterStringSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_STRING_SERIALIZE_OUTPUT_STRING: Mime = "*/*".parse().unwrap();
    }
    /// Create Mime objects for the response content types for TestClientModel
    lazy_static! {
        pub static ref TEST_CLIENT_MODEL_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for TestClassname
    lazy_static! {
        pub static ref TEST_CLASSNAME_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for FindPetsByStatus
    lazy_static! {
        pub static ref FIND_PETS_BY_STATUS_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for FindPetsByTags
    lazy_static! {
        pub static ref FIND_PETS_BY_TAGS_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetPetById
    lazy_static! {
        pub static ref GET_PET_BY_ID_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for UploadFile
    lazy_static! {
        pub static ref UPLOAD_FILE_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetInventory
    lazy_static! {
        pub static ref GET_INVENTORY_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOrderById
    lazy_static! {
        pub static ref GET_ORDER_BY_ID_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetStoreFile
    lazy_static! {
        pub static ref GET_STORE_FILE_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for PlaceOrder
    lazy_static! {
        pub static ref PLACE_ORDER_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetUserByName
    lazy_static! {
        pub static ref GET_USER_BY_NAME_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for LoginUser
    lazy_static! {
        pub static ref LOGIN_USER_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for TestSpecialTags
    lazy_static! {
        pub static ref TEST_SPECIAL_TAGS: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for PostPlainText
    lazy_static! {
        pub static ref POST_PLAIN_TEXT: Mime = "text/plain".parse().unwrap();
    }
   /// Create Mime objects for the request content types for PostUrlEncodedForm
    lazy_static! {
        pub static ref POST_URL_ENCODED_FORM: Mime = "application/x-www-form-urlencoded".parse().unwrap();
    }
   /// Create Mime objects for the request content types for PostXmlFeatures
    lazy_static! {
        pub static ref POST_XML_FEATURES: Mime = "application/xml".parse().unwrap();
    }
   /// Create Mime objects for the request content types for PutPlainText
    lazy_static! {
        pub static ref PUT_PLAIN_TEXT: Mime = "text/plain".parse().unwrap();
    }
   /// Create Mime objects for the request content types for FakeOuterBooleanSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_BOOLEAN_SERIALIZE: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for FakeOuterCompositeSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_COMPOSITE_SERIALIZE: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for FakeOuterNumberSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_NUMBER_SERIALIZE: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for FakeOuterStringSerialize
    lazy_static! {
        pub static ref FAKE_OUTER_STRING_SERIALIZE: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for TestBodyWithQueryParams
    lazy_static! {
        pub static ref TEST_BODY_WITH_QUERY_PARAMS: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for TestClientModel
    lazy_static! {
        pub static ref TEST_CLIENT_MODEL: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for TestEndpointParameters
    lazy_static! {
        pub static ref TEST_ENDPOINT_PARAMETERS: Mime = "application/xml; charset=utf-8".parse().unwrap();
    }
   /// Create Mime objects for the request content types for TestEnumParameters
    lazy_static! {
        pub static ref TEST_ENUM_PARAMETERS: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for TestJsonFormData
    lazy_static! {
        pub static ref TEST_JSON_FORM_DATA: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for TestClassname
    lazy_static! {
        pub static ref TEST_CLASSNAME: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for AddPet
    lazy_static! {
        pub static ref ADD_PET: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for UpdatePet
    lazy_static! {
        pub static ref UPDATE_PET: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for UpdatePetWithForm
    lazy_static! {
        pub static ref UPDATE_PET_WITH_FORM: Mime = "application/x-www-form-urlencoded".parse().unwrap();
    }
   /// Create Mime objects for the request content types for PlaceOrder
    lazy_static! {
        pub static ref PLACE_ORDER: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for CreateUser
    lazy_static! {
        pub static ref CREATE_USER: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for CreateUsersWithArrayInput
    lazy_static! {
        pub static ref CREATE_USERS_WITH_ARRAY_INPUT: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for CreateUsersWithListInput
    lazy_static! {
        pub static ref CREATE_USERS_WITH_LIST_INPUT: Mime = "*/*".parse().unwrap();
    }
   /// Create Mime objects for the request content types for UpdateUser
    lazy_static! {
        pub static ref UPDATE_USER: Mime = "*/*".parse().unwrap();
    }

}
