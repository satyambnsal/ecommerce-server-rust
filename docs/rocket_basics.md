Rocket provides primitives to build web servers and applications with Rust. Rocket provides routing, pre-processing of requests, and post-processing of responses.

Rocket applications are centered around routes and handlers. A route is a combination of:
1. A set of parameters to match an incoming request against.
2. A handler to process the request and return a response.



# Launching
Rocket begins serving requests after being launched, which starts a multi-threaded asynchronous server and dispatches requests to matching routes as they arrive.

