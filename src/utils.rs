/// Enum of verb methods
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

/// Function to get and filter method
/// 
/// ## Example:
/// 
/// ```
/// use malaga_http_utils::utils::*;
/// 
/// struct Verb {
///     method: Methods,
/// }
/// 
/// fn main() {
///     let verb = Verb { method: Methods::GET};
///     
///     let method = get_method(verb.method);
/// 
///     assert_eq!(method, "GET");
/// }
/// ```
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
