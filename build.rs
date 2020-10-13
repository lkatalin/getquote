extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src")
        .inputs(&["src/aesm-proto.proto"])
        .run()
        .expect("Running protoc failed.");
}
