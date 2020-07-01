## Status : In Progress

# Introduction
The idea of this small project is to check how to work with large number of concurrent websockets. The following is the quick overview of the project.

```
+--------------------------------------------+
|  +-------+       +-------------------+     |
|  |       |       |                   |     |
|  |       +<------+  Register /       +<---------------+
|  |       |       |  Unregister /     |     |          |
|  |       |       |  SubscribeToTopic |     |    +-----+----+
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

# API endpoints

The following is the list of API endpoints.

| API              | Description                                                                             | Return                                                                 | Key Inputs                             |
|------------------|-----------------------------------------------------------------------------------------|------------------------------------------------------------------------|----------------------------------------|
| Register         | A client can register to the service of websockets presenting a pre-defined key.        | UUID of the client 403 forbidden 409 Conflict 201 registered / created | API key                                |
| Unregister       | A client can un-register providing the UUID provided by the server during registration. | 200 OK 409 User not found                                              | UUID                                   |
| SubscribeTopic   | A client can subscribe to existing topics                                               | 202 Accepted 403 Forbidden 404 Topic not found                         | String of topics  to subscribe to.     |
| UnSubscribeTopic | A client can Unsubscribe from the existing topics                                       | 202 Accepted 403 Forbidden 404 Topic not found                         | String of topics  to unsubscribe from. |

TODO //
