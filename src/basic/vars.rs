fn vars() {

  // let -> constantes
  // let mut -> variable

  // Para números: i
  // Números positivos sin signo: u
  // Longitud:
  //  8 - 8 bits -> 0 a 255
  //  16 - 16 bits -> 0 a 65536
  //  32 - 32 bits -> 0 a 4,294,967,296
  //  64 - 64 bits
  //  128 - 64 bits
  let age: u8 = 24;

  // Comillas sencillas char
  // Comillas doble string
  let name: &str = "lince";

  // Tarea:
  let max_temp: i8 = 28;
  let min_temp: i8 = -14;

  println!("Hola soy {} y tengo {} años", name, age);
  println!("La temperatura máxima es de: {} º y la minima de: {} º", max_temp, min_temp);
}
