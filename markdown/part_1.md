---
series: "Building an Authentication API in Rust"
title: "How to build authentication api in Rust | Part 1"
description: "Guide to developing an authentication API in Rust using gRPC protocol"
tags: [ "rust", "gRPC", "jwt", "tonic" ]
cover_image: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/tq91kc9g8x96l7cnuwou.png"
published: false
---

## Table of content

1. [Overview of the project](#overview-of-the-project)
2. [Features](#features)
3. [Project Structure](#project-structure)
4. [Creating the project](#creating-the-project)
5. [Setting up the workspace](#setting-up-the-workspace)
6. [Defining the proto file](#defining-the-proto-file)
7. [Implementing the authentication service](#implementing-the-authentication-service)
8. [Implementing the security module](#implementing-the-security-module)
9. [Implementing the database module](#implementing-the-database-module)
10. [Testing the API](#testing-the-api)
11. [Conclusion](#conclusion)

## Overview of the project

The basic idea of authentication is to verify the identity of a user. This process involves validating the user's credentials, such as a username and password, to determine if they are who they claim to be. In this tutorial, we will build an authentication API in Rust using the gRPC protocol. The API will handle user registration, login, password reset, and OTP verification. We will use JWT tokens for authentication and secure passwords with salting.

Registering a new account, resetting passwords such actions would require the user to verify their email address. For this client will make a request for OTP code and the server will send the OTP code to the user's email address. The user will then submit the OTP code to verify their email address. Upon confirmation of the email address, the user will be permitted to perform the action.

We'll also need to store the user information for this I will use a no sql database `MongoDB` and for the gRPC server, I will use the `tonic` library.

## Features

- User SignUp and SignIn
- Password reset
- OTP verification
- Securing password with `PASSWORD_SALTING`

## Project Structure

The project will be organized into the following modules:

1. `auth_api`: The main project directory containing the workspace configuration.
2. `authy`: The authentication service module that handles user registration, login, and password reset.
3. `proto_stub`: The gRPC service definition module that defines the service methods and message types.
4. `security`: The security module that provides functionality for generating and verifying JWT tokens, salting passwords, and OTP verification.
5. `databse`: The database module that provides functionality for interacting with the database.

The following defines the project structure:

```plaintext
├── authy
│   ├── src
│   └── Cargo.toml
├── proto_stub
│   ├── proto
│   │   └── authy
│   │       └── protobuf
│   │           └── main.proto
│   ├── src
│   ├── build.rs
│   └── Cargo.toml
├── security
│   ├── src
│   └── Cargo.toml
├── database
│   ├── src
│   └── Cargo.toml
├── .gitignore
├── Cargo.lock
└── Cargo.toml
```

## Creating the project

Splitting the project into multiple crates allows us to separate concerns and keep the codebase organized. This would also make it easier to test and maintain the code. Because of this we are going to use cargo workspace for this project.

Create a new project and open it in Visual Studio Code.
Also we delte the src directory  in the root of the project.

```bash
cargo new auth_api && cd auth_api && rm -r src && code . 
```

Now edit the `Cargo.toml` file in the root of the project and add the following code:

```toml
# /Cargo.toml

[workspace]
members = []

[workspace.dependencies]

tokio = { version = "1.0", features = ["full"] }
tonic = "0.12.3"
tonic-reflection = "0.12.3"
prost = "0.13.3"
prost-types = "0.13.3"
jsonwebtoken = "9.3.0"
chrono = {  version = "0.4.38", features = ["serde"]}
serde = { version = "1.0.215", features = ["derive"] }
```

Now the Cargo.toml (also called as vitual manifest in this case) file in the root basically defines a workspace rather than a package. The `members` field lists the crates that are part of the workspace. The `workspace.dependencies` field is used to define dependencies that are shared across all crates in the workspace. Intially, we don't have any crates in the workspace so we will leave the `members` field empty.

But we have defined the dependencies that are shared across all crates in the workspace. The `tokio` crate is used for asynchronous programming, `tonic` is used for gRPC, `tonic-reflection` is used for gRPC reflection, `prost` and `prost-types` are used for protocol buffer serialization, `jsonwebtoken` is used for JWT token generation and verification, `chrono` is used for date and time manipulation, and `serde` is used for serialization and deserialization.

Now let's create the `authy`, `proto_stub`, `security`, and `database` crates.
Run the following commands to create the crates:

```bash
cargo new authy --bin && 
cargo new proto_stub --lib && 
cargo new security --lib && 
cargo new database --lib
```

`authy` is the main crate that will contain the authentication service implementation.
`proto_stub` is the crate that will contain the gRPC service definition and it will compile the proto file and expose them to `authy`.
`security` is the crate that will contain the security implementation like generating and verifying JWT tokens, salting passwords, and OTP verification.
`database` is the crate that will contain the database implementation.

Now your virtual manifest should look like this:

```toml
# /Cargo.toml
[workspace]
members = [
    "authy",
    "proto_stub",
    "security",
    "database"
]

[workspace.dependencies]
...
```

Now we have setup the project structure and dependencies. In the next section, we will start defining the service methods and message types in the proto file.
