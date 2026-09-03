//use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

/*
 * Archivo procesador.
 * Quiero que esto reciba mensajes de protocolo de la antena y luego decida qué hacer con cada uno.
 * Tambien debería poder formatear Strings crudos para mandarlos por la antena.
 */

pub fn recibir_mensaje(mensaje: &String) -> Result<()> {
    // falta poner que serde_json ponga los cosos donde van
    // rust da miedo unu

    let v: Value = serde_json::from_str(mensaje)?;
    let t: String = v["type"].to_string();
    println!("{}", t);

    match t.as_str() {
        "\"NEW_USER\"" => msg_new_user(mensaje),
        "\"NEW_STATUS\"" => msg_new_status(mensaje),
        "\"USER_LIST\"" => msg_user_list(mensaje),
        "\"TEXT_FROM\"" => msg_text_from(mensaje),
        "\"JOINED_ROOM\"" => msg_joined_room(mensaje),
        "\"ROOM_USER_LIST\"" => msg_room_user_list(mensaje),
        "\"ROOM_TEXT_FROM\"" => msg_room_text_from(mensaje),
        "\"LEFT_ROOM\"" => msg_left_room(mensaje),
        "\"DISCONNECTED\"" => msg_disconnected(mensaje),
        other => println!("hola"),
    }
    Ok(())
}

fn msg_new_user(mensaje: &String) {
    println!("alguien se unio al servidor");
}

fn msg_new_status(mensaje: &String) {
    println!("alguien cambio de status");
}

fn msg_user_list(mensaje: &String) {
    println!("lista de usuarios");
}

fn msg_text_from(mensaje: &String) {
    println!("alguien escribio texto privado");
}

fn msg_public_text_from(mensaje: &String) {
    println!("alguien escribio texto publico");
}

fn msg_joined_room(mensaje: &String) {
    println!("alguien se unio a la sala");
}

fn msg_room_user_list(mensaje: &String) {
    println!("lista de usuarios de una sala");
}

fn msg_room_text_from(mensaje: &String) {
    println!("alguien escribio texto en una sala");
}

fn msg_left_room(mensaje: &String) {
    println!("alguien se salio de la sala");
}

fn msg_disconnected(mensaje: &String) {
    println!("alguien se desconecto");
}