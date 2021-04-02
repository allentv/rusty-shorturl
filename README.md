# rusty-shorturl

This is a learning project as part of my exploration in Rust

## Background

This is a web application developed using the [Rocket framework](rocket.rs), that supports lookup of a short URL and implement the redirect if already available. Else will redirect to a custom error page with an option to add a new short URL.

## Database Setup

The database changes are managed through [diesel](http://diesel.rs/guides/getting-started/). SQLite is used to keep the complexity low and to get something started. The goal will be to move to Postgres later which should be easy to do with minor changes as diesel is an ORM.

## Implementation

The implemenation consists of running a web application developed with rocket framework using Rust nightly build with Postgres database as the backend.

## Running the Application



## Containerization

There are Dockerfiles available that bundles the application to be run in a container with corresponding `make` commands to run in PROD or DEV mode. The commands have a suffix of `-prod` for PROD environment.
