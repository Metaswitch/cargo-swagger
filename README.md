# cargo swagger

This tool adds a `cargo swagger` subcommand to make it simple to generate Rust crates from [Swagger/OpenAPI](https://swagger.io/) specs.

[![crates.io](https://img.shields.io/crates/v/cargo-swagger.svg)](https://crates.io/crates/cargo-swagger) [![Build Status](https://travis-ci.org/Metaswitch/swagger-rs.svg?branch=master)](https://travis-ci.org/Metaswitch/swagger-rs)

Uses https://github.com/swagger-api/swagger-codegen for the codegen, but wraps it so it's easier for Rust usage.

## Installation

`cargo-swagger` requires that you have Docker installed. See https://docs.docker.com/engine/installation/ for the Docker install instructions.

```sh
$ cargo install cargo-swagger
```
