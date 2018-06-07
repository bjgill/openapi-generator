//! Server implementation of petstore_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use futures::Stream;
use std::collections::HashMap;
use std::io::Error;
use std::marker::PhantomData;
use uuid;
use swagger;
use swagger::{Has, XSpanIdString};

use petstore_api::{Api, ApiError,
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

    /// To test special tags
    fn test_special_tags(&self, client: Client, context: &C) -> Box<Future<Item=TestSpecialTagsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("test_special_tags({:?}) - X-Span-ID: {:?}", client, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Get some XML
    fn get_xml_features(&self, context: &C) -> Box<Future<Item=GetXmlFeaturesResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_xml_features() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Post some plaintext
    fn post_plain_text(&self, body: String, context: &C) -> Box<Future<Item=PostPlainTextResponse, Error=ApiError>> {
        let context = context.clone();
        println!("post_plain_text(\"{}\") - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Post some form data in url encoded format
    fn post_url_encoded_form(&self, param1: String, param2: String, param3: Option<String>, context: &C) -> Box<Future<Item=PostUrlEncodedFormResponse, Error=ApiError>> {
        let context = context.clone();
        println!("post_url_encoded_form(\"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", param1, param2, param3, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Post some xml
    fn post_xml_features(&self, xml_object: XmlObject, context: &C) -> Box<Future<Item=PostXmlFeaturesResponse, Error=ApiError>> {
        let context = context.clone();
        println!("post_xml_features({:?}) - X-Span-ID: {:?}", xml_object, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Put some optional plaintext
    fn put_plain_text(&self, body: Option<String>, context: &C) -> Box<Future<Item=PutPlainTextResponse, Error=ApiError>> {
        let context = context.clone();
        println!("put_plain_text({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// an endpoint with uuid-type headers
    fn uuid_header(&self, x_uuid_header: uuid::Uuid, context: &C) -> Box<Future<Item=UuidHeaderResponse, Error=ApiError>> {
        let context = context.clone();
        println!("uuid_header({:?}) - X-Span-ID: {:?}", x_uuid_header, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn fake_outer_boolean_serialize(&self, body: Option<bool>, context: &C) -> Box<Future<Item=FakeOuterBooleanSerializeResponse, Error=ApiError>> {
        let context = context.clone();
        println!("fake_outer_boolean_serialize({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }


    fn fake_outer_composite_serialize(&self, outer_composite: Option<OuterComposite>, context: &C) -> Box<Future<Item=FakeOuterCompositeSerializeResponse, Error=ApiError>> {
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


    fn test_body_with_query_params(&self, query: String, user: User, context: &C) -> Box<Future<Item=TestBodyWithQueryParamsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("test_body_with_query_params(\"{}\", {:?}) - X-Span-ID: {:?}", query, user, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// To test \"client\" model
    fn test_client_model(&self, client: Client, context: &C) -> Box<Future<Item=TestClientModelResponse, Error=ApiError>> {
        let context = context.clone();
        println!("test_client_model({:?}) - X-Span-ID: {:?}", client, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Fake endpoint for testing various parameters 假端點 偽のエンドポイント 가짜 엔드 포인트 
    fn test_endpoint_parameters(&self, unknown_base_type: models::object, context: &C) -> Box<Future<Item=TestEndpointParametersResponse, Error=ApiError>> {
        let context = context.clone();
        println!("test_endpoint_parameters({:?}) - X-Span-ID: {:?}", unknown_base_type, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// To test enum parameters
    fn test_enum_parameters(&self, enum_header_string_array: Option<&Vec<String>>, enum_header_string: Option<String>, enum_query_string_array: Option<&Vec<String>>, enum_query_string: Option<String>, enum_query_integer: Option<i32>, unknown_base_type: Option<models::object>, context: &C) -> Box<Future<Item=TestEnumParametersResponse, Error=ApiError>> {
        let context = context.clone();
        println!("test_enum_parameters({:?}, {:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", enum_header_string_array, enum_header_string, enum_query_string_array, enum_query_string, enum_query_integer, unknown_base_type, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// test json serialization of form data
    fn test_json_form_data(&self, unknown_base_type: models::object, context: &C) -> Box<Future<Item=TestJsonFormDataResponse, Error=ApiError>> {
        let context = context.clone();
        println!("test_json_form_data({:?}) - X-Span-ID: {:?}", unknown_base_type, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// To test class name in snake case
    fn test_classname(&self, client: Client, context: &C) -> Box<Future<Item=TestClassnameResponse, Error=ApiError>> {
        let context = context.clone();
        println!("test_classname({:?}) - X-Span-ID: {:?}", client, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Add a new pet to the store
    fn add_pet(&self, pet: Pet, context: &C) -> Box<Future<Item=AddPetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("add_pet({:?}) - X-Span-ID: {:?}", pet, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Deletes a pet
    fn delete_pet(&self, pet_id: i64, api_key: Option<String>, context: &C) -> Box<Future<Item=DeletePetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_pet({}, {:?}) - X-Span-ID: {:?}", pet_id, api_key, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Finds Pets by status
    fn find_pets_by_status(&self, status: &Vec<String>, context: &C) -> Box<Future<Item=FindPetsByStatusResponse, Error=ApiError>> {
        let context = context.clone();
        println!("find_pets_by_status({:?}) - X-Span-ID: {:?}", status, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Finds Pets by tags
    fn find_pets_by_tags(&self, tags: &Vec<String>, context: &C) -> Box<Future<Item=FindPetsByTagsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("find_pets_by_tags({:?}) - X-Span-ID: {:?}", tags, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Find pet by ID
    fn get_pet_by_id(&self, pet_id: i64, context: &C) -> Box<Future<Item=GetPetByIdResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_pet_by_id({}) - X-Span-ID: {:?}", pet_id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Update an existing pet
    fn update_pet(&self, pet: Pet, context: &C) -> Box<Future<Item=UpdatePetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("update_pet({:?}) - X-Span-ID: {:?}", pet, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Updates a pet in the store with form data
    fn update_pet_with_form(&self, pet_id: i64, name: Option<String>, status: Option<String>, context: &C) -> Box<Future<Item=UpdatePetWithFormResponse, Error=ApiError>> {
        let context = context.clone();
        println!("update_pet_with_form({}, {:?}, {:?}) - X-Span-ID: {:?}", pet_id, name, status, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// uploads an image
    fn upload_file(&self, pet_id: i64, additional_metadata: Option<String>, file: Box<Future<Item=Option<Box<Stream<Item=Vec<u8>, Error=Error> + Send>>, Error=Error> + Send>, context: &C) -> Box<Future<Item=UploadFileResponse, Error=ApiError>> {
        let context = context.clone();
        println!("upload_file({}, {:?}, ) - X-Span-ID: {:?}", pet_id, additional_metadata, context.get().0.clone());
        let _ = file; //Suppresses unused param warning
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Delete purchase order by ID
    fn delete_order(&self, order_id: String, context: &C) -> Box<Future<Item=DeleteOrderResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_order(\"{}\") - X-Span-ID: {:?}", order_id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Returns pet inventories by status
    fn get_inventory(&self, context: &C) -> Box<Future<Item=GetInventoryResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_inventory() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Find purchase order by ID
    fn get_order_by_id(&self, order_id: i64, context: &C) -> Box<Future<Item=GetOrderByIdResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_order_by_id({}) - X-Span-ID: {:?}", order_id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Returns store details
    fn get_store_file(&self, context: &C) -> Box<Future<Item=GetStoreFileResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_store_file() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Place an order for a pet
    fn place_order(&self, order: Order, context: &C) -> Box<Future<Item=PlaceOrderResponse, Error=ApiError>> {
        let context = context.clone();
        println!("place_order({:?}) - X-Span-ID: {:?}", order, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create user
    fn create_user(&self, user: User, context: &C) -> Box<Future<Item=CreateUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("create_user({:?}) - X-Span-ID: {:?}", user, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Creates list of users with given input array
    fn create_users_with_array_input(&self, user: &Vec<models::User>, context: &C) -> Box<Future<Item=CreateUsersWithArrayInputResponse, Error=ApiError>> {
        let context = context.clone();
        println!("create_users_with_array_input({:?}) - X-Span-ID: {:?}", user, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Creates list of users with given input array
    fn create_users_with_list_input(&self, user: &Vec<models::User>, context: &C) -> Box<Future<Item=CreateUsersWithListInputResponse, Error=ApiError>> {
        let context = context.clone();
        println!("create_users_with_list_input({:?}) - X-Span-ID: {:?}", user, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Delete user
    fn delete_user(&self, username: String, context: &C) -> Box<Future<Item=DeleteUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_user(\"{}\") - X-Span-ID: {:?}", username, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Get user by user name
    fn get_user_by_name(&self, username: String, context: &C) -> Box<Future<Item=GetUserByNameResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_user_by_name(\"{}\") - X-Span-ID: {:?}", username, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Logs user into the system
    fn login_user(&self, username: String, password: String, context: &C) -> Box<Future<Item=LoginUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("login_user(\"{}\", \"{}\") - X-Span-ID: {:?}", username, password, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Logs out current logged in user session
    fn logout_user(&self, context: &C) -> Box<Future<Item=LogoutUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("logout_user() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Updated user
    fn update_user(&self, username: String, user: User, context: &C) -> Box<Future<Item=UpdateUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("update_user(\"{}\", {:?}) - X-Span-ID: {:?}", username, user, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
