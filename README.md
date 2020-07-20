# Rust notebook

Hi - In the notebook, are some loosely coupled projects that i made during the course of me learning rust. I will continue to update anything worth archiving that can help a larger audience.
List of items are categorized roughly as below.

## General
Rust langauge samples for various concepts.

## Command line utilities

Rust is popular in its own way for writing utilities for command line. The clap library provides a neat way to extract arguments from any command line run programmes / services / adapters etc.

## Async and network
In rust, async & await would need a lot of practise and in the folder you would see some examples of using ...

 - Some simple usage of Async await - futures
 - Simple examples of Tokio run time, spawn  
 - TcpStream ( converting tcpstreams { sockets } as Streams ) & socket based client server exchange of a file
 - TcpUtils ( codecs ) based simple chat / echo server
 - Converting u8 vectors into streams.

## Concurrency & parallelism
- Threading support and simple usage of cross beam channels
- Ray-on usage with image decoding & resize along with actix-web.

## Database
- A sample program that demonstrates CRUD with Postgresql using diesel.rs ORM adapter, migration of schema using diesel-cli. Sakila sample db is used as sample. I move some int2's around as int4s in the db schematic.
- [Someday] Noria, Neo4J and influxDB usage.

## Microservices
- actix-web-mongodb is expected to demonstrate the usage of ..
	- Mongo driver of rust (CRUD operations of a record. R2d2 polling is avoided since the connection polling is part of the mongodb rust driver.)
    - Actix web framework - CRUD APIs with db ( along with postman json )
    - Static serving of files.
    - Create user with actos for db create user request.
    - Client CRUD API testing app based on react-admin
- warp-web-sockets-redis is expected to demonstrate the usage of ..
  - websockets based on tokio-tungstenite ( comes with warp )
  - Cors filtering
  - Redis interfaces with multiplexed connections.
- gRPC tonic implementation for totp verification
  - tonic based grpc server generation
  - totp verification based on google authorization package
  - APIs for register and verify totp
  - A sample web app to demonstrate usage based on grpc-web
  - [Envoy Proxy](https://www.envoyproxy.io/) configuration for interfacing between server and client.

## Data
- Apache arrow rust library - Simple SQL command.
  - Simple data loading from SQL command using arrow library.

## QA
- Quickcheck - sample usage for automated unit testing based on function template.
- Criterion - Sample usage for collecting benchmarks for sample runs of reading, re-sizing and saving two image variants.

## Iot
- MQTT client with paho.mqtt.rust lib with example of thingspeak
