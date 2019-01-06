use std::str;
use serde_json;

///trait to serialize o
pub trait NetworkObj {
    fn deserialize_network(&self, network: &[u8]) -> Self;
}

///Enum of methods
#[derive(Serialize, Deserialize, Debug)]
pub enum Methods {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

///struct to define headers
#[derive(Serialize, Deserialize, Debug)]
pub struct Headers {
    content_type: String,
    accept: String,
    accept_charset: String,
    accept_language: String,
    accept_ranges: String,
    accept_encoding: String,
    access_control_allow_credentials: bool,
    access_control_allow_headers: String,
    access_control_allow_methods: String,
    access_control_allow_origin: String,
    access_control_expose_headers: String,
    access_control_max_age: String,
    access_control_request_headers: String,
    access_control_request_method: Methods,
}

///struct to implement deserialize request
#[derive(Serialize, Deserialize, Debug)]
pub struct Req{
    headers: Headers,
    method: String,

}


impl NetworkObj for Req {
    fn deserialize_network(&self, network: &[u8]) -> Self {
        let network_string = str::from_utf8(network).unwrap();
        let des_network: Req = serde_json::from_str(network_string).unwrap();
        
        des_network
    }
}

