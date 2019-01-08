//! # Malaga HTTP Utils
//! 
//! `malaga_http_utils` 

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate serde;
extern crate bincode;

pub mod utils;

///trait to serialize objects networks
pub trait NetworkObj {
    fn serialize_to_struct(network_bin: &[u8]) -> Self;
    fn serialize_to_binary(network_obj: Self) -> Vec<u8>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    impl NetworkObj for TestReq {
        fn serialize_to_struct(req: &[u8]) -> Self {
            let req_string = str::from_utf8(req).unwrap();
            let ser_req: TestReq = serde_json::from_str(req_string).unwrap();
            
            ser_req
        }

        fn serialize_to_binary(req: TestReq) -> Vec<u8> {
            let ser_req: Vec<u8> = bincode::serialize(&req).unwrap();

            ser_req
        } 
}

    #[derive(Serialize, Deserialize, Debug)]
        struct TestHeader {
            #[serde(rename = "Content-Type")]
            content_type: String,
            #[serde(rename = "Authorization")]
            pub authorization: String,
        }

    #[derive(Serialize, Deserialize, Debug)]
    struct Body {
        user: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TestReq {
        headers: TestHeader,
        method: utils::Methods,
        body: Body,
    }

    #[test]
    fn it_should_serialize_to_strcut() {

        let resq = b"{
            \"headers\": {
                \"Content-Type\": \"application/json\",
                \"Authorization\": \"Basis 1ddmcdd\"
            },
            \"method\": \"POST\",
            \"body\": {
                \"user\":\"test\"
            }
        }";

        let resq_serialized: TestReq = TestReq::serialize_to_struct(resq);
        let method = utils::get_method(resq_serialized.method);

        assert_eq!(resq_serialized.body.user, "test");
        assert_eq!(method, "POST");
    }

    #[test]
    fn it_should_serializa_to_binary() {
        let resq = b"{
            \"headers\": {
                \"Content-Type\": \"application/json\",
                \"Authorization\": \"Basis 1ddmcdd\"
            },
            \"method\": \"POST\",
            \"body\": {
                \"user\":\"test\"
            }
        }";

        let resq_struct_ser: TestReq = TestReq::serialize_to_struct(resq);

        println!("struct: {:#?}", resq_struct_ser);

        let resq_binary_ser: Vec<u8> = TestReq::serialize_to_binary(resq_struct_ser);

        let req_string: String = bincode::deserialize(&resq_binary_ser[..]).unwrap();
        println!("binary {:#?}", req_string);
    }
}
