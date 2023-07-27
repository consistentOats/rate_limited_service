# Running this project
To run this project, [Install Rust](https://www.rust-lang.org/tools/install) then run `cargo run` in the this project's directory - it should start an http server running on localhost:8080

You can then use the included postman collection or just curl against the following endpoints:

POST localhost:8080/vault

GET localhost:8080/vault/items

PUT localhost:8080/vault/items/:id

Please be sure you include a bearer token when making this HTTP requests (it does not need to be a valid token, it just needs to not be blank).

The responses you get should include headers to expose some data about how you are being rate limited:
"x-ratelimit-remaining" tells you how many requests you have remaining in the rate limiting window.
"x-ratelimit-retry-after" tells you how long (in seconds) you need to wait before you can make another request.
