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
    let t: String = v["type"].to_string().;
    println!("{}", t);

    match t.as_str() {
        "NEW_USER" => msg_new_user(),
        "NEW_STATUS" => msg_new_status(),
        "USER_LIST" => msg_user_list(),
        "TEXT_FROM" => msg_text_from(),
        "JOINED_ROOM" => msg_joined_room(),
        "ROOM_USER_LIST" => msg_room_user_list(),
        "ROOM_TEXT_FROM" => msg_room_text_from(),
        "LEFT_ROOM" => msg_left_room(),
        "DISCONNECTED" => msg_disconnected(),
        other => println!("hola"),
    }
    Ok(())
}


// Imaginemos que hay un private antes de todos los metodos msg_algo(), y que nadie nunca jamas los va a usar mas que en la funcion recibirMensaje()

fn msg_new_user() {
    println!("alguien se unio al servidor");
}

fn msg_new_status() {
    println!("alguien cambio de status");
}

fn msg_user_list() {
    println!("lista de usuarios");
}

fn msg_text_from() {
    println!("alguien escribio texto privado");
}

fn msg_public_text_from() {
    println!("alguien escribio texto publico");
}

fn msg_joined_room() {
    println!("alguien se unio a la sala");
}

fn msg_room_user_list() {
    println!("lista de usuarios de una sala");
}

fn msg_room_text_from() {
    println!("alguien escribio texto en una sala");
}

fn msg_left_room() {
    println!("alguien se salio de la sala");
}

fn msg_disconnected() {
    println!("alguien se desconecto");
}