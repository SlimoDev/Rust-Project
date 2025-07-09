/*struct Main_Window{
    escenarioCargado: Escenario,
    musicaCargada: Musica
}
*/
#![allow(non_snake_case)]

use soloud::*;
use std::io;
use std::process;
use std::path::Path;

fn main() {
    //ReproducirAudio("Assets/Musica/Hotel.wav", true);
    println!("Título del juego");
    //Variable de inputs
    let mut input: String = "".to_string();
    //Cargar menú principal
    MenuPrincipalLogica(input);

}
//opciones para cargar o guardar partida y cerrar el juego
fn CargarMenuPrincipal() {
    println!("Seleccione una opción: ");
    println!("1. Nueva Partida\n2. Cargar Partida\n3. Cerrar");
}
//Se carga el menú principal
fn MenuPrincipalLogica(mut input: String){
    //Se verifica que el input sea el correcto, sino, se repite
    loop{
        CargarMenuPrincipal();
        input = RecibirInput("Seleccione una opción con números");
        //Revisa si la respuesta del usuario es la esperada en el menú y rompe el bucle
        if  (ValidarTexto(&input, "1") || ValidarTexto(&input, "2") || ValidarTexto(&input, "3")){
            break;
        }
    }
    //Identifica que opción usó el usuario
    match input.as_str() {
        "1" => println!("Empezando nueva partida..."), //Flata implementar
        "2" => println!("Menú de cargar partida..."), //Falta implementar
        "3" => CerrarJuego(),
        _ => MenuPrincipalLogica(input),
    }
}
fn CargarPartidaMenu(){
    println!("Seleccione archivo de guardado");
    //Se deberán mostrar los 3 archivos de guardadi
}
fn CerrarJuego(){
    println!("Cerrando...");
    process::exit(0);
}
//Carga y reproducide archivos de audio TODAVÍA NO FUNCIONA
fn ReproducirAudio(ruta: &str, bucle: bool) {
    let mut sl = match Soloud::default() {
        Ok(soloud) => soloud,
        Err(e) => {
            eprintln!("Error: No se pudo inicializar Soloud: {}", e);
            return;
        }
    };

    let mut wav = audio::Wav::default();

    if let Err(e) = wav.load(&Path::new(ruta)) {
        eprintln!("Error: No se pudo cargar el archivo '{}': {}", ruta, e);
        return;
    }

    // Configurar bucle
    wav.set_looping(bucle);

    let handle = sl.play(&wav);
    println!("Reproduciendo '{}' en bucle {}", ruta, bucle);

    // Detener reproducción
    //sl.stop(handle);
    //println!("Reproducción terminada");
}
fn RecibirInput(mensaje: &str) -> String{
    println!("{}", mensaje);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea");
    input.trim().to_string()
}

//Valida si el texto input del jugador es el esperado para avanzar o realizar una acción
fn ValidarTexto(textoIngresado: &str , mut textoEsperado: &str) ->  bool {

    let textIn = textoIngresado.to_lowercase().trim().to_string();
    let textEsp = textoEsperado.to_lowercase().trim().to_string();
    return textEsp == textIn;
}
fn ValidarInt() {

}
fn ValidarFloat(){

}
fn ValidarString(){

}
