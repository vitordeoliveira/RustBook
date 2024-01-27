# Final Project: Building a Multithreaded Web Server

Here is our plan for building the web server:

1. Learn a bit about TCP and HTTP.
1. Listen for TCP connections on a socket.
1. Parse a small number of HTTP requests.
1. Create a proper HTTP response.
1. Improve the throughput of our server with a thread pool.

## Building a Single-Threaded Web Server

We’ll start by getting a single-threaded web server working. Before we begin,
let’s look at a quick overview of the protocols involved in building web
servers. The details of these protocols are beyond the scope of this book, but
a brief overview will give you the information you need.

The two main protocols involved in web servers are Hypertext Transfer Protocol
(HTTP) and Transmission Control Protocol (TCP). Both protocols are
request-response protocols, meaning a client initiates requests and a server
listens to the requests and provides a response to the client. The contents of
those requests and responses are defined by the protocols.

TCP is the lower-level protocol that describes the details of how information
gets from one server to another but doesn’t specify what that information is.
HTTP builds on top of TCP by defining the contents of the requests and
responses. It’s technically possible to use HTTP with other protocols, but in
the vast majority of cases, HTTP sends its data over TCP. We’ll work with the
raw bytes of TCP and HTTP requests and responses.

## Turning Our Single-Threaded Server into a Multithreaded Server

Right now, the server will process each request in turn, meaning it won’t
process a second connection until the first is finished processing. If the
server received more and more requests, this serial execution would be less and
less optimal. If the server receives a request that takes a long time to
process, subsequent requests will have to wait until the long request is
finished, even if the new requests can be processed quickly. We’ll need to fix
this, but first, we’ll look at the problem in action.
