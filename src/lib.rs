/*!
This is an implementation of the public api of racetime.gg. See their documentation for more
https://github.com/racetimeGG/racetime-app/wiki/Public-API-endpoints

This crate owes a lot to https://github.com/saintdev/speedrun-api

The design is taken therefrom, and also quite a bit of code.
Unlike that crate, this is significantly less flexible. Everything is built around
reqwest, and many assumptions are made about racetime.gg's API, such as that
all endpoints require no auth and are GET endpoints.

Also I haven't implemented anything to help with pagination.
 */

extern crate async_trait;
extern crate core;
extern crate reqwest;
extern crate url;

#[macro_use]
extern crate derive_builder;

pub mod client;
pub mod endpoint;
pub mod endpoints;
mod err;
pub mod types;
