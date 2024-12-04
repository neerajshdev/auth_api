---
series: "Step by step guide to build authentication api in Rust using gRPC"
title: "Setting up the project | Part 1"
description: "Guide to developing an authentication API in Rust using gRPC protocol"
tags: [ "rust", "gRPC", "jwt", "tonic" ]
cover_image: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/tq91kc9g8x96l7cnuwou.png"
published: false
---

## Introduction

Welcome to the first part of our comprehensive guide on building an authentication API in Rust using gRPC! In this series, we will walk you through the process of creating a robust and secure authentication system from scratch. Whether you are a seasoned Rustacean or new to the language, this tutorial is designed to help you understand and implement key concepts in authentication, gRPC, and secure communication.

By the end of this series, you will have a fully functional authentication API that supports user registration, login, password reset, and OTP verification. We will leverage powerful libraries such as `tonic` for gRPC, `jsonwebtoken` for JWT handling, and `MongoDB` for data storage. Let's embark on this exciting journey to master authentication in Rust!

## Table of Contents

1. [Overview of the Project](#overview-of-the-project)
2. [Features](#features)
3. [Project Structure](#project-structure)
4. [Creating the Project](#creating-the-project)

## Overview of the Project

The basic idea of authentication is to verify the identity of a user. This process involves validating the user's credentials, such as a username and password, to determine if they are who they claim to be. In this tutorial, we will build an authentication API in Rust using the gRPC protocol. The API will handle user registration, login, password reset, and OTP verification. We will use JWT tokens for authentication and secure passwords with salting.

Registering a new account and resetting passwords will require the user to verify their email address. The client will request an OTP code, and the server will send the OTP code to the user's email address. The user will then submit the OTP code to verify their email address. Upon confirmation of the email address, the user will be permitted to perform the action.

We'll also need to store the user information. For this, we will use a NoSQL database, `MongoDB`, and for the gRPC server, we will use the `tonic` library.

We will make use of the json web token (JWT) for authentication. JWT is a compact, URL-safe means of representing claims to be transferred between two parties. The claims in a JWT are encoded as a JSON object that is used as the payload of a JSON Web Signature (JWS) structure or as the plaintext of a JSON Web Encryption (JWE) structure, enabling the claims to be digitally signed or integrity protected with a Message Authentication Code (MAC) and/or encrypted.

You can know more about JWT [here](https://jwt.io/introduction/).

We will use HMAC SHA-256 for signing the JWT token. HMAC SHA-256 is a cryptographic hash function that produces a fixed-size output (256 bits) for a given input. It is a secure way to verify the integrity and authenticity of the data.
This algorithm used a single key for both signing and verifying the token. In this project the key will be known only to the server. So the server can verify the token and also generate the token. This approach guarantees that the tokens cannot be tampered with or forged by unauthorized parties. By using JWT tokens as session tokens, we can confidently ensure secure and reliable user authentication.

## Features

- User SignUp and SignIn
- Password reset
- OTP verification
- Securing password with `PASSWORD_SALTING`

## Project Structure

The project will be organized into the following modules:

1. `authy`: The authentication service crate that handles user registration, login, and password reset.
2. `proto_stub`: The gRPC service definition creat that defines the service methods and message types.
3. `security`: The security crate that provides functionality for generating and verifying JWT tokens, salting passwords, and OTP verification.
4. `database`: The database crate that provides functionality for interacting with the database.

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

## Creating the Project

Splitting the project into multiple crates allows us to separate concerns and keep the codebase organized. This would also make it easier to test and maintain the code. Because of this, we are going to use cargo workspace for this project.

Create a new project and open it in Visual Studio Code. Also, delete the src directory in the root of the project.

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
chrono = {  version = "0.4.38", features = ["serde"] }
serde = { version = "1.0.215", features = ["derive"] }
```

Now the Cargo.toml (also called as virtual manifest in this case) file in the root basically defines a workspace rather than a package. The `members` field lists the crates that are part of the workspace. The `workspace.dependencies` field is used to define dependencies that are shared across all crates in the workspace. Initially, we don't have any crates in the workspace, so we will leave the `members` field empty.

But we have defined the dependencies that are shared across all crates in the workspace. The `tokio` crate is used for asynchronous programming, `tonic` is used for gRPC, `tonic-reflection` is used for gRPC reflection, `prost` and `prost-types` are used for protocol buffer serialization, `jsonwebtoken` is used for JWT token generation and verification, `chrono` is used for date and time manipulation, and `serde` is used for serialization and deserialization.

Now let's create the `authy`, `proto_stub`, `security`, and `database` crates. Run the following commands to create the crates:

```bash
cargo new authy --bin && 
cargo new proto_stub --lib && 
cargo new security --lib && 
cargo new database --lib
```

`authy` is the main crate that will contain the authentication service implementation. `proto_stub` is the crate that will contain the gRPC service definition and it will compile the proto file and expose them to `authy`. `security` is the crate that will contain the security implementation like generating and verifying JWT tokens, salting passwords, and OTP verification. `database` is the crate that will contain the database implementation.

Now your virtual manifest should have member packages like this:

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
 # common dependencies for sub-crates
```

Now we have set up the project structure and dependencies. 
You can now run the following command to build the project:

```bash
cargo build
```
And to build the specif crate you can run the following command:

```bash 
cargo build -p authy
```

replace `authy` with the crate name you want to build.


## Conclusion

In this part of the tutorial, we have successfully set up the initial project structure and defined the necessary dependencies for our authentication API. By creating separate sub-crates for different functionalities, we have ensured that our codebase remains organized and maintainable. This modular approach will make it easier to test and extend the project in the future.

In the next part of the tutorial, we will focus on defining the gRPC service methods and message types in the proto file. We will also write the `build.rs` script to compile the protobuf definitions and generate the Rust stubs for our server. This will lay the groundwork for implementing the core functionality of our authentication service. Stay tuned for more detailed steps and explanations!

If you have any questions or feedback, feel free to leave a comment below. I'll be happy to help you out. Stay tuned for the next part of the tutorial!
