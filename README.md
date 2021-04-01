# rusty-shorturl

This is a learning project as part of my exploration in Rust

## Background

This is a web application developed using the [Rocket framework](rocket.rs), that supports lookup of a short URL and implement the redirect if already available. Else will redirect to a custom error page with an option to add a new short URL.

## Implementation

The implemenation consists of running a web application developed with rocket framework using Rust nightly build with Postgres database as the backend.

## Running the Application



## Containerization

There are Dockerfiles available that bundles the application to be run in a container with corresponding `make` commands to run in PROD or DEV mode. The commands have a suffix of `-prod` for PROD environment.
