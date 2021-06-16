fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "src/grpc/pb";

    if !std::path::Path::new(dir).exists() {
        // We need to check auth errors and so on so we need to ensure that authorization not done yet.
        std::fs::create_dir(dir).expect("can't create directory");
    }

    let proto_dir = "proto";
    for file in [
        "proto/users.proto",
        "proto/records.proto",
        "proto/sources.proto",
    ]
    .iter()
    {
        tonic_build::configure()
            .out_dir(dir)
            .compile(&[file], &[&proto_dir])?;
    }
    Ok(())
}
