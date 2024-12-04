---
series: "Building an Authentication API in Rust"
title: "Buiding a gRPC AUTH API in Rust| Part 1"
description: "Guide to developing an authentication API in Rust using gRPC protocol"
tags: rust, grpc, jwt, docker
cover_image: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/tq91kc9g8x96l7cnuwou.png"
published: true
---

## Introduction

In this post, I'll explain how to develop an authentication API in Rust using the gRPC protocol. The authentication will be based on JWT tokens. We'll also create a docker file to deploy the application in a docker container. Lastly, I'll provide instructions on how to deploy the app.

## Features

- User Registration
- User login
- Password reset/FORGOT password
- OTP verification
- Password Hashing

Crate a new project and open it in vs code editor

```bash
cargo new auth_api && cd auth_api && code . 
```

## Defining the GRPC scheme for the API

Create a auth.proto file

```proto
// /proto/auth.proto

syntax = "proto3";
package tutorial.api.auth;

service AuthService {
}
```

## SignUp a new account Flow

1. The user initiates a request to sign up for a new account. The server acknowledges this request by generating an OTP code, which it sends to the user's email along with a temporary token. This token includes the `req_perms` claim, detailing the permissions the user seeks—in this instance, the permission to create a new account.

2. In the subsequent request, the user submits the valid OTP together with the temporary token to confirm their email address. Upon receiving the OTP code and the temporary token, the server validates the OTP. If it is correct, the server creates a new temporary token with a `perms` claim and moves all permissions from the `req_perms` to the `perms` claim. The server then sends a response with this new temporary token.

3. To create a new account user can send their details along with the temporary token.

### InitiateSignUp gRPC method

Let's define the `InitiateSignUp` in `/proto/auth.proto/AuthService`.

```proto
rpc InitiateSignUp(InitiateSignupParams) 
returns (InitiateSignUpResponse);
```

Let's define the request and response message body for this method

```proto
message InitiateSignupParams {
    string email = 1;
}

message InitiateSignUpResponse {
    bool success = 1;
    string message = 2;
    string token = 3;
}
```

The following is an example of a successful response message.

```json
{
    "success": true,
    "message": "Otp was sent to your email",
    "token": {
        "iss": "auth_api",
        "sub": "<email_id>",
        "exp": "<epoch-time>",
        "iat": "<epoch-time>",
        "req_perms": ["CreateAccount"]
    }
}
```

`token`: This is a temporary token that includes the req_perms (requested permissions) claim. `CreateAccount` is a necessary permission for creating a new account.

### VerifyOtp gRPC method

Next, create the `VerifyOtp` method to verify the email and grant the requested permissions defined in the temporary token.

Write the following in `/proto/auth.proto/AuthService`

```proto
rpc VerifyOtp(VerifyOtpParams) returns (VerifyOtpResponse);
```

Let's define the request and response message body for this method

```proto
message VerifyOtpParams { 
    string token = 1;
    string otp = 2;
}
message VerifyOtpResponse { 
    bool success = 1;
    string message = 2;
    string token = 3;
}
```

The following describes the success response message for the VerifyOtp method

```json
{
    "success": true,
    "message": "Your email is verified",
        "token":  { 
        "iss": "auth_api",
        "sub": "<email_id>",
        "exp": "<epoch-time>",
        "iat": "<epoch-time>",
        "perms": ["CreateAccount"]
    }
}
```

The success response includes a temporary token that grants CreateAccount permission.

### CreateAccount gRPC method

Next create a `CreateAccount` method to create the user account with the username, password and other details.

write the following in  `/proto/auth.proto/AuthService`

```proto
rpc CreateAccount(CreateAccountParams) returns (CreateAccountResponse);
```

let's define the request and response message body for this method

```proto
message CreateAccountParams { 
    string username = 1;
    string password = 2;
    // other details
    string full_name = 3;
    Gender gender = 4;
    string birth_date = 5;
}

message CreateAccountResponse { 
    bool success = 1;
    string message = 2;
    string session_token = 3;
    User user = 4;
}

message User {
    string username = 1;
    string email = 2;
    string full_name = 3;
    Gender gender = 4;
    string birth_date = 5;
}

enum Gender {
    MALE = 0;
    FEMALE = 1;
    OTHER = 2;
}
```

Following describes the success response message for the CreateAccount method

```json
{
    "success": true,
    "message": "Account created successfully",
    "session_token": {
        "iss": "auth_api",
        "sub": "<username>",
        "exp": <epoch-time>,
        "iat": <epoch-time>,
        "perms": ["ApiAccess"]
    }, 
    "user": {
        "username": "<username>",
        "email": "<email>",
        "full_name": "<full_name>",
        "gender": "<gender>",
        "birth_date": "<birth_date>"
    }
}
```

So far we have created all the methods for creating a new account.

### Login gRPC method

Next, we need to define the Login method to login to the account.
User can use either their email or username to login to their account.

First, we need to define the `Login` method to login to the account.

append the following in `/proto/auth.proto/AuthService`

```proto
rpc Login(LoginParams) returns (LoginResponse);
```

Let's define the request and response message body for this method

```proto
message LoginParams { 
    string email_or_username = 1;
    string password = 2;
}
message LoginResponse { 
    bool success = 1;
    string message = 2;
    string session_token = 3;
    User user = 4;
}
```

Success response would have a session token as follows

```json
{
    "success": true,
    "message": "Login successful",
    "session_token": {
        "iss": "auth_api",
        "sub": "<username>",
        "exp": "<epoch-time>",
        "iat": "<epoch-time>",
        "perms": ["Session"]
    }, 
    "user": {
        "username": "<username>",
        "email": "<email>",
        "full_name": "<full_name>",
        "gender": "<gender>",
        "birth_date": "<birth_date>"
    }
}
```

### InitiateResetPassword gRPC method

This method will accept the email id and send a password reset token to the user's email id.
Also, server will send a otp code to the user's email id.

append `/proto/auth.proto/AuthService`

```proto
rpc InitiateResetPassword(InitiateResetPasswordParams) returns (InitiateResetPasswordResponse);
```

Let's define the parameters and response message body for this method

*append `/proto/auth.proto`*

```proto

message InitiateResetPasswordParams { 
    string email_id = 1;
}

message InitiateResetPasswordResponse { 
    bool success = 1;
    string message = 2;
    string token = 3;
}

```

A success response for the `InitiateResetPassword` method will have a password reset token as follows

```json
{
  "success": true,
  "message": "Password reset token sent to your email",
  "token":  {
    "iss": "auth_api",
    "sub": "<email_id>",
    "exp": <epoch-time>,
    "iat": <epoch-time>,
    "req_perms": ["ResetPassword"]
  }
}
```

the req_perms claim in the token describes that the user will be granted the permission to reset the password after giving right otp code to the `VerifyOtp` method. We already have `VerifyOtp` method in the previous section.

Let's define the `ResetPassword` method to reset the user password.
ResetPassword method will accept the password reset token and the new password.

append `/proto/auth.proto/AuthService`

```proto
rpc ResetPassword(ResetPasswordParams) returns (ResetPasswordResponse);
```

Let's define the parameters and response message body for this method

*append `/proto/auth.proto`*

```proto

message ResetPasswordParams { 
    string token = 1;
    string new_password = 2;
}

message ResetPasswordResponse { 
    bool success = 1;
    string message = 2;
}
```

A success response for the `ResetPassword` method will have a message as follows

```json
{
  "success": true,
  "message": "Password reset successful"
}
```

## Conclusion

We have defined the GRPC scheme for the authentication API. For now it supports create account, login, reset password in case if the user forgot their password. This also includes the OTP verification to make sure the user is using the valid email id. </br>

In the next part, we will start coding the API in Rust.
See you in the next part.
Bye.
