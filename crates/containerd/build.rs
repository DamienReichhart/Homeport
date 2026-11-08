//! Generates the containerd API's client code from the vendored `.proto`
//! files with the vendored protoc, so building needs nothing installed
//! (ADR 0020).

const PROTOS: &[&str] = &[
    "github.com/containerd/containerd/api/services/containers/v1/containers.proto",
    "github.com/containerd/containerd/api/services/content/v1/content.proto",
    "github.com/containerd/containerd/api/services/events/v1/events.proto",
    "github.com/containerd/containerd/api/services/images/v1/images.proto",
    "github.com/containerd/containerd/api/services/introspection/v1/introspection.proto",
    "github.com/containerd/containerd/api/services/namespaces/v1/namespace.proto",
    "github.com/containerd/containerd/api/services/snapshots/v1/snapshots.proto",
    "github.com/containerd/containerd/api/services/streaming/v1/streaming.proto",
    "github.com/containerd/containerd/api/services/tasks/v1/tasks.proto",
    "github.com/containerd/containerd/api/services/transfer/v1/transfer.proto",
    "github.com/containerd/containerd/api/services/version/v1/version.proto",
    "github.com/containerd/containerd/api/types/transfer/imagestore.proto",
    "github.com/containerd/containerd/api/types/transfer/progress.proto",
    "github.com/containerd/containerd/api/types/transfer/registry.proto",
    "github.com/containerd/containerd/api/types/transfer/streaming.proto",
    "github.com/containerd/containerd/api/events/container.proto",
    "github.com/containerd/containerd/api/events/content.proto",
    "github.com/containerd/containerd/api/events/image.proto",
    "github.com/containerd/containerd/api/events/namespace.proto",
    "github.com/containerd/containerd/api/events/snapshot.proto",
    "github.com/containerd/containerd/api/events/task.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let well_known = protoc_bin_vendored::include_path()?;
    let mut config = tonic_prost_build::Config::new();
    config.protoc_executable(protoc);
    config.protoc_arg("--experimental_allow_proto3_optional");
    // `Any`'s type URL is the message's full name, as containerd's typeurl writes it.
    config.enable_type_names();
    let root = std::path::PathBuf::from("proto");
    let protos: Vec<_> = PROTOS.iter().map(|p| root.join(p)).collect();
    tonic_prost_build::configure()
        .build_server(false)
        .emit_rerun_if_changed(false)
        .compile_with_config(config, &protos, &[root, well_known])?;
    Ok(())
}
