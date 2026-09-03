mod procesador;

fn main() {
    let x = procesador::recibir_mensaje(
        &r#"{ "type": "USER_LIST",
              "users": { "Kimberly": "ACTIVE",
              "Luis": "BUSY",
              "Fernando": "AWAY",
              "Antonio": "ACTIVE" } }"#.to_string()
            );
    println!("Hola soy el cliente :D!");
}
