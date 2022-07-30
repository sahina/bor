# Transaction Service

## Requirements

To test gRPC endpoints, a gRPC client is needed.

`grpcurl` is a useful and simple grpc curl.

```shell
brew install grpcurl
```

### Test trx server

```shell
# relative path for -import-path
cd crates/transaction

# execute method
grpcurl -plaintext -import-path ./proto -proto transaction.proto -d '{"name": "Tonic"}' '[::1]:50052' trx.RecordAuthorized/RecordTransaction
```

## Docker

### Trx server

```shell
# build
docker build -t bor_trx -f docker/Dockerfile .

# run
docker run --rm --name bor_trx -d -p 50052:50051 bor_trx
```