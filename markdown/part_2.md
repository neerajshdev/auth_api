---
title: "How to build authentication api in Rust | Part 2"
description: "Guide to developing an authentication API in Rust using gRPC protocol"
tags: [ "rust", "gRPC", "jwt", "tonic" ]
cover_image: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/tq91kc9g8x96l7cnuwou.png"
published: false
---

## Table of Contents

- [Creating the proto definition for the authentication service](#creating-the-proto-definition-for-the-authentication-service)
  - [Service Definition](#service-definition)
  - [OTP Challenge Messages](#otp-challenge-messages)
  - [OTP Confirmation Messages](#otp-confirmation-messages)
  - [Sign Up Messages](#sign-up-messages)
  - [Sign In Messages](#sign-in-messages)
  - [User Message Type](#user-message-type)

## Creating the proto definition for the authentication service

In the previous part, we created the project structure and defined our workspace with its sub-projects (crates). In this part, we will define the proto file for the authentication service. The proto file defines the service methods and message types that the gRPC server will use to communicate with the client.

### Service Definition

Now let's create the main.proto file in the proto_stub/proto/authy/protobuf directory. The main.proto file defines the service methods and message types for the authentication service.

```protobuf
# /proto_stub/proto/authy/protobuf/main.proto

syntax = "proto3";
package authy.protobuf;

import "google/protobuf/timestamp.proto";

service AuthService {
    rpc CreateOtpChallenge(CreateOtpChallengeRequest) returns (CreateOtpChallengeResponse);
    rpc ConfirmOtp(ConfirmOtpRequest) returns (ConfirmOtpResponse);

    rpc SignUp(SignUpRequest) returns (SignUpResponse);
    rpc SignIn(SignInRequest) returns (SignInResponse);
}
```

The main.proto file defines the AuthService service with the following methods:

- `CreateOtpChallenge`: This method creates an OTP challenge for the user to verify their email address. The method takes a `CreateOtpChallengeRequest` message as input and returns a `CreateOtpChallengeResponse` message as output.
- `ConfirmOtp`: This method confirms the OTP challenge for the user to verify their email address. The method takes a `ConfirmOtpRequest` message as input and returns a `ConfirmOtpResponse` message as output.
- `SignUp`: This method registers a new user account. The method takes a `SignUpRequest` message as input and returns a `SignUpResponse` message as output.
- `SignIn`: This method signs in a user account. The method takes a `SignInRequest` message as input and returns a `SignInResponse` message as output.

### OTP Challenge Messages

Now let's define the `CreateOtpChallengeRequest` and `CreateOtpChallengeResponse` message types.

```protobuf
# /proto_stub/proto/authy/protobuf/main.proto
message CreateOtpChallengeRequest {
    string email = 1;
    ActionPurpose action_purpose = 2;

    enum ActionPurpose {
        SIGN_UP = 0;
        SIGN_IN = 1; // sign in via otp
        RESET_PASSWORD = 2;
    }
}

message CreateOtpChallengeResponse {
    string challenge_token = 1;
}
```

The `CreateOtpChallengeRequest` message type defines the fields for creating an OTP challenge. The message contains the following fields:

- `email`: The user's email address.
- `action_purpose`: The purpose of the action that requres the two factor auth via otp, such as signing up, signing in, or resetting the password. The `ActionPurpose` enum defines the possible values for the action purpose.

The `CreateOtpChallengeResponse` message type defines the fields for the OTP challenge response. The message contains the challenge token that the user will use to verify their email address.

### OTP Confirmation Messages

Now let's define the `ConfirmOtpRequest` and `ConfirmOtpResponse` message types.

```protobuf
# /proto_stub/proto/authy/protobuf/main.proto
message ConfirmOtpRequest {
    string challenge_token = 1;
    string otp_code = 2;
}

message ConfirmOtpResponse {
    bool success = 1;
    string message = 2;
    optional string action_token = 3;
    Error error = 4;

    enum Error  {
        INVALID_OTP = 0;
        EXPIRED_OTP = 1;
        MAX_ATTEMPTS_REACHED = 2;
    }
}
```

The `ConfirmOtpRequest` message type defines the fields for confirming the OTP challenge. The message accepts the challenge token and the OTP code that the user submits to verify their email address.

The `ConfirmOtpResponse` message type defines the fields for the ConfirmOtp response. The message contains the success status, message, action token, and error type. The `Error` enum defines the possible error types for the OTP verification process. The error types include `INVALID_OTP`, `EXPIRED_OTP`, and `MAX_ATTEMPTS_REACHED`.

### Sign Up Messages

Now let's define the `SignUpRequest` and `SignUpResponse` message types.

```protobuf
# /proto_stub/proto/authy/protobuf/main.proto
message SignUpRequest {
    string email = 1;
    string username = 2;
    string password = 3;
    string fullname = 4;
    string gender = 5;
    google.protobuf.Timestamp birthdate = 6;
    string action_token = 7;
}

message SignUpResponse {
    bool success = 1;
    string message = 2;
    optional string session_token = 3;
    Error error = 4;
    User user = 5;

    enum Error {
        INVALID_ACTION_TOKEN = 0;
        EMAIL_ALREADY_EXISTS = 1;
        USERNAME_ALREADY_EXISTS = 2;
    }
}
```

The `SignUpRequest` message type defines the fields for registering a new user account. The message contains the fields for user's username and password, and basic information of the user and a action token which is required for the user to perform the signup action.

The `SignUpResponse` message type defines the fields for the SignUp response. The message contains the success status, message, session_token, user message type that describe the current signin user.  and error type. The `Error` enum defines the possible error types for the SignUp process. The error types include `INVALID_ACTION_TOKEN`, `EMAIL_ALREADY_EXISTS`, and `USERNAME_ALREADY_EXISTS`. session token is optional and might be none in case of error.

### Sign In Messages

Now let's define the `SignInRequest` and `SignInResponse` message types.

```protobuf
# /proto_stub/proto/authy/protobuf/main.proto
message SignInRequest {
    oneof signInBy { 
        string email = 1;
        string username = 2;
        string action_token = 4;
    }
    optional string password = 3;
}

message SignInResponse {
    bool success = 1;
    string sessin_token = 2;
    string message = 3;
    optional User user = 4;
    Error error = 5;

    enum Error {
        INVALID_CREDENTIALS = 0;
        INVALID_ACTION_TOKEN = 1;
    }
}
```

The `SignInRequest` message type defines the fields for signing in a user account. User can sign in using email or username or via otp so we have defined oneof field for the same. oneof field is used to define a field that can have only one value from a set of fields.

Next we have password field which is required for the user to sign in with email or username. This field is optional and might be none in case of sign in via otp.

The `SignInResponse` message type defines the fields for the SignIn response. The message contains the success status, message, session_token, user message type that describe the current signin user.  and error type. The `Error` enum defines the possible error types for the SignIn process. The error types include `INVALID_CREDENTIALS`, `INVALID_ACTION_TOKEN`.

### User Message Type

Also, defines the User message type that contains the user information. This message type will be used to return the user information in the SignUp and SignIn responses with session token.
Note that we are using the `google.protobuf.Timestamp` type for the birthdate and created_at fields. So don't forget to import the timestamp.proto file at the top of the main.proto file.

```protobuf
# /proto_stub/proto/authy/protobuf/main.proto
message User {
    string username = 1;
    string email = 2;
    string fullname = 3;
    string gender = 4;
    google.protobuf.Timestamp birthdate = 5;
    google.protobuf.Timestamp created_at = 6;
}
```

So far, we have defined the proto file for the authentication service with the service methods and message types. In the next part, we will write the build.rs script to generate the Rust code from the proto file using the `tonic-build` crate.

See you in the next part. Happy coding! 🚀
