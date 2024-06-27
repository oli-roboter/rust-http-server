use std::fmt::{ Display, Debug, Formatter, Result as FmtResult };
use std::net::TcpStream;
use std::io::{ Write, Result as IoResult };
use super::status_code::StatusCode;
/*
body can be JSON, html, etc or nothing, so let's use a String which can hold all of these
*/
#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    /*
    1. read comment at the end of the file for why not use the Display trait directly
    2. pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {} -> the code below static dispatch:
    instead of saying that stream is a TcpStream type that has the write trait, it uses some generalisation
    This will be resolved at compile time, but will make compiling slower, as the compiler needs to figure out that
    the write function is for a TcpStream in this case.
    Not sure what the advantage of that is, thay say it is testing, but I'm not sure
     */

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

/*
Deleting the below code in favour of the send function in the Response implementation.
This is because when sending the response to the stream, the contents of the Response will be converted to a string using the 
fmt method in the Display trait.
It will copy the contents of the Response into a string, which gets allocated to the heap and then sent to the stream.
That might be a problem if the body is enourmous, like a whole main.js file, as it will copy the heap memory and allocate it to another
address in the heap, which will use a lot of resource.
The better approach is to send the string to the strem directly, without copying it. This is done in the send function
So, before we were writing to the Formatter, which would allocate a new string with all the Response contents to the heap and then send it to the stream
Now, we are sending the string directly to the stream and no allocation is needed


impl Display for Response {
    /*
    Implementing Display for Response will allow to use the write! macro only passing in the response like:
    write!(&self, "{}", response) and it will format the response as implemented through the fmt function
    */
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
*/
