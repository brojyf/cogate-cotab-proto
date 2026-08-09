fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "proto/cogate/cotab/v1/internal.proto",
        "proto/cogate/notify/v1/internal.proto",
        "proto/rusti2/v1/object_storage.proto",
    ];

    for proto in protos {
        println!("cargo:rerun-if-changed={proto}");
    }

    tonic_build::configure().compile_protos(&protos, &["proto"])?;
    Ok(())
}
