# containerd's API

The `.proto` files of the containerd API services and types Homeport uses, copied unchanged from [containerd](https://github.com/containerd/containerd) (`api/`, Apache-2.0), with `google/rpc/status.proto` from [googleapis](https://github.com/googleapis/googleapis) (Apache-2.0) and `fieldpath.proto` from [gogo/protobuf](https://github.com/gogo/protobuf) (BSD-3-Clause). Their licence headers stay in each file.

`build.rs` compiles them with the protoc of `protoc-bin-vendored` (ADR 0020), so building Homeport needs no protoc installed. To update the API, replace these files with the ones from a containerd release and list any new one in `build.rs`.
