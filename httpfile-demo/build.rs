fn main() {
    println!("cargo:rerun-if-changed=index.http");
    httpfile_build::configure()
        .httpfile("index.http")
        .compile()
        .unwrap();
    println!("Code generation for index.http successful!!");
}
