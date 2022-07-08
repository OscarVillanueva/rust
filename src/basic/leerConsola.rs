fn console() {

  println!("Por favor introduce tu nombre: ");
  
  // &str -> es un tipo de primitivo
  // String -> es un objeto o clase
  // String::new() -> cadena vacía
  // String::from() -> inicializar una cadena
  // "Cadena".to_string() -> inicializar una cadena
  let mut name: String = String::new();
  let mut raw_age: String = String::new();
  
  // std -> conexión con el SO
  // io -> inputs / outputs
  // stdin -> lectura
  // read_line -> leer una linea desde consola
  // unwrap() -> es para manejo de errores
  // &mut name -> mutar / actualizar el valor en la dirección de memoria que tenga name
  std::io::stdin().read_line(&mut name).unwrap();
  name = name.trim().to_string();
  
  // Obtener la edad por medio de la consola
  println!("Por favor introduce tu edad: ");
  std::io::stdin().read_line(&mut raw_age).unwrap();

  // Convertir esa edad a un número
  let age: u8 = raw_age.trim().parse().unwrap();

  println!("Hola: {}, bienvenido de {} años", name, age);

  // Tarea
  let mut hm_name: String = String::new();
  let mut hm_city: String = String::new();

  println!("Ingresa tu nombre: ");
  std::io::stdin().read_line(&mut hm_name).unwrap();
  hm_name = hm_name.trim().to_string();

  println!("Ingresa tu cuidad: ");
  std::io::stdin().read_line(&mut hm_city).unwrap();
  hm_city = hm_city.trim().to_string();

  println!("Bienvenido {} de {}", hm_name, hm_city)


}
