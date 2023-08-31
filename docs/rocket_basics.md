Rocket provides primitives to build web servers and applications with Rust. Rocket provides routing, pre-processing of requests, and post-processing of responses.

Rocket applications are centered around routes and handlers. A route is a combination of:
1. A set of parameters to match an incoming request against.
2. A handler to process the request and return a response.



# Launching
Rocket begins serving requests after being launched, which starts a multi-threaded asynchronous server and dispatches requests to matching routes as they arrive.



* Any number of dynamic path segments are allowed. A path segment can be of any type, including your own, as long as the type implements the `FromParam` trait. we call these types *parameter guards*.


* You can also match against multiple segments by using `<param..>` in a route path. The type of such parameters, known as segments guards, must implement `FromSegments`. A segment guard must be the final component of a path.


* To serve static files, use `FileServer` type from rocket
  
  `rocket.mount("/public", FileServer::from("static/"))`

* A component of a route can be fully ignored by using `<_>`



## Forwarding

* Routes are attempted in increasing rank order. Rocket chooses a default ranking from -12 to -1, but a route's rank can also be manually set with the `rank` attribute.

Default ranking: If a rank is not explicitly specified, Rocket assigns a default rank. The default rank prefers static segments over dynamic segments in both path and queries: the more static a route's path and query are, the higher its precedence.

## Request Guards

Request guards are one of Rocket's most powerful instruments. As the name might imply, a request guard protects a handler from being called erroneously based on information contained in an incoming request. More specifically, a request guard is a type that represents an arbitrary validation policy. The validation policy is implemented through the `FromRequest` trait. Every type that implements `FromRequest` is a request guard.

Request guards centralize policies, resulting in a simpler, safer, and more secure applications.


