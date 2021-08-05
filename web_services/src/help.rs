pub fn help() {
    println!("1) Sobre el Redis-Server:\n");
    println!("cargo run\n\n");
    println!("2) Sobre el Web-Server:\n");
    println!("cargo run 127.0.0.1:8081 127.0.0.1:8082\n\n");
    println!("3) En una pestaña de un navegador ingresar:\n");
    println!("127.0.0.1:8081\n\n\n\n");
    println!("NOTA: 127.0.0.1:8082 depende del config server del Redis-Server");
}
