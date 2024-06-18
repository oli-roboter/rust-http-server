// use super::method::{ Method, MethodError };
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{ Display, Debug, Formatter, Result as FmtResult };
use std::str;
use std::str::Utf8Error;

// Example HTTP request: GET /search?name=abc&sort=1HTTP/1.1\r\n...HEADERS
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    // Convers the buffer of u8 received in the stream to a Request
    // fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
}

// implementing the TryFrom trait for the Request type
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    /*
    function copied from the TryFrom trait (click on the import) in line 645
    fn try_from(value: T) -> Result<Self, Self::Error>;
    Adapting the copied function for this case:
     */
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /* 
        Ways to make the same thing, from more verbose to succint

        1.match str::from_utf8(buf) {
            Ok(request) => {}
            Err(_) => {
                return Err(ParseError::InvalidEncoding);
            }
        }

        2. let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding));
        */
        let request = str::from_utf8(buf)?;

        /*
        // Verbose way of getting the next work, followed by concise way
        match get_next_word(request) {
            Some((method, request)) => {}
            None => {
                return Err(ParseError::InvalidRequest);
            }
        }
        */
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(idx) = path.find('?') {
            query_string = Some(path[idx + 1..].to_string());
            path = &path[..idx];
        }

        Ok(Self {
            path: path.to_string(),
            query_string,
            method,
        })
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidEncoding,
    InvalidProtocol,
}

impl ParseError {
    // method that is used in the trait implementation of the Debug and Display traits
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
        }
    }
}

/*  
To make the ParseError more idiomatic - so it follows the standar Error and forces us to meet basic expectations for errors 
- the Error trait needs to be implemented
*/
impl Error for ParseError {}
// When implementing the Error trait, it asks for the Debug + Display traits.
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// impl From<MethodError> for ParseError {
//     fn from(value: MethodError) -> Self {
//         Self::InvalidMethod
//     }
// }

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, c) in request.chars().enumerate() {
        // if char is a space or enf of line (\r)
        if c == ' ' || c == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }
    None
}
