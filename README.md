# Rust-http-server

## Project summary

This is a basic http server built with Rust as a learning project.

To run this you need to have the [rust programming language installed](https://www.rust-lang.org/learn/get-started) on your machine.

Go to the project root and type `cargo run` and this will spin up a server listening on `127.0.0.1:8080`

By pasting the above address in your browser, a web page should render. If the `/hello` path is appended to the address, another page with a message should show. Any other paths should result in a `404 Not Found` status displayed on the browser

Additionally, the server accepts a query string after the initial path, such as `127.0.01.8080?a=hello&b=2&c&d=&e===&d=7&d=abc`

In the terminal it will display the following:

```
Listening on 127.0.0.1:8080
11Received a request: GET /?/a=hello&b=2&c&d=&e===&d=7&d=abc HTTP/1.1
Host: 127.0.0.1:8080
Connection: keep-alive
sec-ch-ua: "Not/A)Brand";v="8", "Chromium";v="126", "Google Chrome";v="126"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "macOS"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br, zstd
Accept-Language: en-GB,en-US;q=0.9,en;q=0.8


[src/server.rs:43] &request = Request {
    path: "/",
    query_string: Some(
        QueryString {
            data: {
                "d": Multiple(
                    [
                        "",
                        "7",
                        "abc",
                    ],
                ),
                "e": Single(
                    "==",
                ),
                "b": Single(
                    "2",
                ),
                "c": Single(
                    "",
                ),
                "/a": Single(
                    "hello",
                ),
            },
        },
    ),
    method: GET,
}
```

### Limitations and next steps

- Currently the server only has the `GET` method implemented
- The server is sinlge threaded
- The server does not deal with headers

Objectives for the futurefoo is to implement Header handling and other Methods such as `POST` and make the server use multi threading
