# Martus

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [Help](#help)
- [License](#license)

## Description

Yet another Open Source School Management Software

## Getting Started

### Dependencies

The following sections explores various environment (device) dependencies and the technology stack used in building the project. Note that this is subjected to incremental change

#### Environment Dependencies

The following are required to run the application locally

- [Docker](https://www.docker.com) A platform designed to help developers build, share and run containerized applications
- [PostgreSQL](https://postgresql.org) - Powerful and Open Source Object-relational database management system
- [Apache Kafka](https://kafka.apache.org) - Open source distributed event streaming platform
- [nx](https://nx.dev) - a smart, fast and extensible build system for monorepos
- [NodeJS](https://nodejs.org) - Open source JavaScript runtime environment

#### Technology Stack

- [Typescript](https://www.typescriptlang.org) - a strongly typed programming language that builds on JavaScript
- [Rust](https://www.rust-lang.org) - A programming language empowering everyone to build reliable and efficient softwares
- [NodeJS >=v18.xx](https://nodejs.org) Open source JavaScript runtime environment
- [NPM >=v9.xx](https://nodejs.org) or Yarn - JavaScript package manager

### Installing

The following sections will focus of running the application locally

- Clone the code repository

```sh
git clone https//github.com/opeolluwa/martus
cd martus
```

### Executing program

The project is best managed with [nx](https://nx.dev) editor plugin

_TO run a project from the command line, consider installing nx globally `npm  i -g nx`_

#### Add a new Rust binary to the workspace

```sh
nx generate @monodon/rust:binary <project-name>
```

#### Add a new Rust library to the workspace

```sh
nx generate @monodon/rust:library <library-name>
```

#### Run a project

```sh
nx run <project-name>:run
```

#### Test a project

```sh
nx run <project-name>:test
```

#### Lint a project

```sh
nx run <project-name>:lint
```

see `nx --help` for more options

## Documentation

See [martus.docs](./Martus.md)

## Help

Need Help? use the repository [wiki](https://github.com/opeolluwa/martus/wiki)

## License

This project is licensed under the BSD 2-Clause License - see the [LICENSE](./LICENSE) file for details
