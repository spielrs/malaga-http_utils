///Enum of network methods
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

///Function to get and filter method
pub fn get_method(method: Methods) -> String {
    match method {
        Methods::GET => String::from("GET"),
        Methods::HEAD => String::from("HEAD"),
        Methods::POST => String::from("POST"),
        Methods::PUT => String::from("PUT"),
        Methods::DELETE => String::from("DELETE"),
        Methods::CONNECT => String::from("CONNECT"),
        Methods::OPTIONS => String::from("OPTIONS"),
        Methods::TRACE => String::from("TRACE"),
        Methods::PATCH => String::from("PATCH"),
    }
}
