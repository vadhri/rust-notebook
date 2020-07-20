The sample project contains a GRPC server and client.

## Server
In order to host the server, go into the server directory in root and run the following command.
Pre-requisites
- Have a mongodb instance running on standard port 27017
- Have the port 8089 available.

```
cargo run --bin server
```

In order to verify the hosted service on the port, please use [bloomrpc](https://github.com/uw-labs/bloomrpc) gui client.

## Client
In order to have the client code generated, please follow the steps as part of [grpc-web](https://github.com/grpc/grpc-web) to have the environment set up. The following command would generate the client stubs.

```
protoc -I=../ ../server/proto/user_totp.proto \
--js_out=import_style=commonjs:generated \
--grpc-web_out=import_style=commonjs,mode=grpcwebtext:generated
```

Once the code is generated using webpack ( using the following commands build, host. ), the same should be used as listed in the client/*

```
$ npm install

...

$ npx webpack

...

$ python3 -m http.server
```

## Proxy
grpc-web cannot directly talk to the grpc server, hence it needs a proxy to translate the requests. Envoy proxy configuration is provided with the project.
Please run the following command in a seperate window, if the set up is not local, the endpoint should be changed in the yaml file of the grpc server.

```
envoy --config-path ./config.yaml
```
