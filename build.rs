fn main() {
    capnpc::CompilerCommand::new()
        .file("spore.capnp")
        .run()
        .expect("Cap'n Proto schema compilation failed");
}
