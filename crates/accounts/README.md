# Accounts Service

## Requirements

To test gRPC endpoints, a gRPC client is needed.

`grpcurl` is a useful and simple grpc curl.

```shell
brew install grpcurl
```

### Test auth server

```shell
# relative path for -import-path
cd crates/authorization

# execute method
grpcurl -plaintext -import-path ./proto -proto accounts.proto -d '{"name": "Tonic"}' '[::1]:50051' auth.Authorizer/AuthorizeTransaction
```

## Docker

### Auth server

```shell
# build
docker build -t bor_auth -f docker/Dockerfile .

# run
docker run --rm --name bor_auth -d -p 50051:50051 bor_auth
```