# Bank of Rust - BoR

CQRS/ES based trading library.

## Docker

```shell
cd docker
docker compose --profile base up --abort-on-container-exit
docker compose down --rm
```

## Events

```rust
// define event
define_event!(UserSignedUp, {
  email: String,
  full_name: String
});

// create event
let event = event!(UserSignedUp, {
  email: "homer@simpsons.com",
  full_name: "Homer Simpson"
});
```

## Commands

```rust
// define command
define_command!(SignupUser, {
  email: String,
  full_name: String,
  password: String
});

// create command
let command = command!(SignupUser, {
  email: "homer@simpsons.com",
  full_name: "Homer Simpson",
  password: "secret"
});
```

## Aggregates

```rust
fn handler(command: CreateUser) -> Vec<Event> {
    // todo
}

define_aggregate!(UserRegistration, {
  events: [UserSignedUp],
  commands: [SignupUser],
  handlers: [handler]
});
```
