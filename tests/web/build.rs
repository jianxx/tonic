fn main() {
    let protos = &["proto/test.proto"];

    tonic_prost_build::configure()
        .compile_protos(protos, &["proto"])
        .unwrap();

    protos
        .iter()
        .for_each(|file| println!("cargo:rerun-if-changed={file}"));
}
