//use serde_json;

/*
 * Archivo procesador.
 * Quiero que esto reciba mensajes de protocolo de la antena y luego decida qué hacer con cada uno.
 * Tambien debería poder formatear Strings crudos para mandarlos por la antena.
 */

fn recibirMensaje(mensaje: &String) {
    struct Info {
        r#type: String,
        username: String,
        status: String,
        users: Vec<String>,
        text: String,
        roomname: String,
    };

    // falta poner que serde_json ponga los cosos donde van
    // rust da miedo unu


    let tipo: str = "aqui iría el tipo del mensaje jeje";

    match tipo {
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
}


// Imaginemos que hay un private antes de todos los metodos msg_algo(), y que nadie nunca jamas los va a usar mas que en la funcion recibirMensaje()

fn msg_new_user() {

}

fn msg_new_status() {

}

fn msg_user_list() {

}

fn msg_text_from() {

}

fn msg_public_text_from() {

}

fn msg_joined_room() {

}

fn msg_room_user_list() {

}

fn msg_room_text_from() {

}

fn msg_left_room() {

}

fn msg_disconnected() {

}