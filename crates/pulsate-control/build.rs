//! Compiles the admin gRPC proto into client and server stubs in `OUT_DIR`.

fn main() {
    // Point prost/tonic at the vendored protoc unless the environment already
    // pins one, so no system `protoc` install is required (CI, `cargo install`).
    if std::env::var_os("PROTOC").is_none() {
        if let Ok(protoc) = protoc_bin_vendored::protoc_bin_path() {
            std::env::set_var("PROTOC", protoc);
        }
    }

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_protos(&["proto/admin.proto"], &["proto"])
        .expect("compile proto/admin.proto");
    println!("cargo:rerun-if-changed=proto/admin.proto");
}
