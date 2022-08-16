fn main() {
    httpfile_build::configure()
        .httpfile("index.http")
        .compile()
        .unwrap();
    println!("Build successful!!");
}
