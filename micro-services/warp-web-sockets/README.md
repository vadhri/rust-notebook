# Introduction
The idea of this small project is to check how to work with large number of concurrent websockets. The following is the quick overview of the project.

```
+--------------------------------------------+
|  +-------+       +-------------------+     |
|  |       |       |                   |     |
|  |       +<------+  Register /       +<---------------+
|  |       |       |  Unregister /     |     |          |
|  |       |       |                   |     |    +-----+----+
|  |       |       |                   |     |    |          |
|  |       |       +-------------------+     |    |  Client  |
|  | Redis |       +-------------------+     |    |          |
|  |       |       |                   |     |    +-----+----+
|  |       |       |                   |     |          |
|  |       |       |  WebSockets       +<---------------+
|  |       |       |                   |     |
|  +-------+       +-------------------+     |
+--------------------------------------------+
```

The project would demonstrate the following.
- Warp websocket connect, send / recv messages
- Routes to be Configured
- Cors confguration
- Redis interfaces
- A sample for connecting over WebSocket
- Artillery usage with WebSockets


# API endpoints

The following is the list of API endpoints.

| API              | Description                                                                             | Return                                                                 | Key Inputs                             |
|------------------|-----------------------------------------------------------------------------------------|------------------------------------------------------------------------|----------------------------------------|
| Register         | A client can register to the service of websockets presenting a pre-defined key.        | UUID of the client 403 forbidden 409 Conflict 201 registered / created | API key                                |
| Unregister       | A client can un-register providing the UUID provided by the server during registration. | 200 OK 409 User not found                                              | UUID                                   |
| broadcast        | Broadcast to all connected users.                                                       | 200 OK 409 User not found                                              | UUID                                   |

TODO //

Build the source with the following command.
```
cargo run
```

Test html should be able to help with sample commands. 
