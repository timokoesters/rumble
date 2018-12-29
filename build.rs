use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
	    out_dir: "src",
	    input: &["protos/mumble.proto"],
	    includes: &["protos"],
	    customize: Customize {
	      ..Default::default()
	    },
	}).expect("Failed to use protoc_rust");
}
