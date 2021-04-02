[![Build Status](http://jenkins.paz.ienza.tech/job/url-shortener/job/master/badge/icon)](http://jenkins.paz.ienza.tech/job/url-shortener/job/master/)

# URL Shortener

This project was generated with Cargo and developed in rust. 

With an active MongoDB server, this file can be executed and connected serving as a RESTFul api, allowing URLs to be 'shortened' by generating a hash for each URL and saving it the database, redirecting any requests made to the endpoint {BASE_URI}/{HASH} to the coordinating URL that was stored. All hashes are unique 6 digit codes, and the same URL cannot be stored twice.

## Development server

Running cargo run will set up a development version of the system based off of the configurations in [Rocket.toml]{Rocket.toml}

## Running unit tests

TODO
