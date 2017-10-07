# Try:
#   cargo install protobuf
#   cargo install grpc-compiler
#   GOBIN=~/bin go get -u github.com/golang/protobuf/protoc-gen-go


mkdir -p src/proto
protoc --rust_out=src/proto      proto/service.proto
protoc --rust-grpc_out=src/proto proto/service.proto

mkdir -p go/src/foolproof.io/broadcast/proto
protoc --go_out=plugins=grpc:go/src/foolproof.io/broadcast/proto -I proto/ proto/service.proto
