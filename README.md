# rust-demo-webserver


This repo contains two very basic web servers written in Rust.

Both webservers have a `GET /hello` endpoint.
`server1` is calling `server2` and pass through it's result to the callee.
`server2` is just returning a string


### Intention

The sole purpose of this repo is to have to services that communicate with each other
to have a test service e.g. in a Kubernetes setup where two services should work together.