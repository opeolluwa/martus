# Martus Central Authentication Server

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [Help](#help)
- [Authors](#authors)
- [Version History](#version-history)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Description

gRPC service for microservices authentication

## Getting Started

### Dependencies

_The required system dependencies are listed thus:_

_The application is developed and tested on [Ubuntu](https://ubuntu.com) 22.04, please consult guides that correspond to your operating system, especially in installing the dependencies_

- [Rust](https://www.rust-lang.org) - a language empowering everyone to build fast and efficient software
- [Apache Kafka](https://kafka.apache.org) - Open source distributed event streaming platform mission-critical application
- [Docker](https://docker.com) - A platform designed to help developers to build, share and run containerized applications.
- [PostgreSQL](https://www.postgresql.org) -Open source Object Relational Database Management System.
- [grpcurl](https://github.com/fullstorydev/grpcurl) - A tool like cUrl but for gRPC - a command line tool for interacting with gRPC

### Installing

Before installing, ensure the [#dependencies](#dependencies) have been met

To begin, clone the project
Change the directory into the project

```sh
git clone https://github.com/opeolluwa/martus
cd martus/auth
cargo run
```

If all goes well, you should get a log like this
`2023-11-05T06:22:48.068002Z  INFO auth: Starting server. server_address=0.0.0.0:5001`

### Executing program

- Create a `.env` file and populate it appropriately, the `.env.example` contains the guide it this

- Perform the health check

```sh
grpcurl --vv --plaintext -proto ./proto/auth.proto -d '{}' 0.0.0.0:5001 martus
_auth.Auth/HealthCheck
```

if all goes well, you should get a response like this:

```txt
Resolved method descriptor:
// see the grpc server status
rpc HealthCheck ( .martus_auth.HealthCheckRequest ) returns ( .martus_auth.HealthCheckResponse );

Request metadata to send:
(empty)

Response headers received:
content-type: application/grpc
date: Sun, 05 Nov 2023 06:51:16 GMT

Estimated response size: 28 bytes

Response contents:
{
  "status": "Ok",
  "message": "Service up and running"
}

Response trailers received:
(empty)
Sent 1 request and received 1 response
```

see [proto.sh](./proto.sh) for other tests

## Documentation

TBC ..

## License

The project is licensed and distributed under [BSD 2-Clause License](./LICENSE)
