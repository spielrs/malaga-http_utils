//! # Malaga HTTP Utils
//! 
//! `malaga_http_utils` 

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate serde;

pub mod utils;

///trait to serialize objects networks
pub trait NetworkObj {
    fn deserialize_network<'a>(network_obj: Self) -> &'a[u8];
    fn serialize_network(network_bin: &[u8]) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    impl NetworkObj for TestReq {
        fn serialize_network(req: &[u8]) -> Self {
            let req_string = str::from_utf8(req).unwrap();
            let ser_req: TestReq = serde_json::from_str(req_string).unwrap();
            
            ser_req
        }

        fn deserialize_network<'a>(req: TestReq) -> &'a[u8] {
            let des_req: &[u8] = serde::de::Deserialize::deserialize(req).unwrap();

            des_req
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
    fn it_should_deserialize_request() {

        let resq = b"{
            \"headers\": {
                \"Content-Type\": \"Application\",
                \"Authorization\": \"Basis 1ddmcdd\"
            },
            \"method\": \"POST\",
            \"body\": {
                \"user\":\"test\"
            }
        }";

        let resq_deserialized: TestReq = TestReq::deserialize_network(resq);
        let method = utils::get_method(resq_deserialized.method);

        assert_eq!(resq_deserialized.body.user, "test");
        assert_eq!(method, "POST");
    }
}
