fn conditionals() {
  // https://www.reddit.com/r/rust/comments/6ln5du/mut_mut_why_is_this_required/djv99go/

  let mut str_age: String = String::new();
  let age: u8;

  println!("Ingresa tu edad: ");
  // &mut es para delegarle a read_line el acceso/permiso para mutar el valor que este
  // en esa referencia de memoria
  // Ademas va hacer un append a la dirección
  std::io::stdin().read_line(&mut str_age).unwrap();


  // String to int
  // Age no es mutable porque estoy reasignando más no mutando / cambiando el valor
  age = str_age.trim().parse().unwrap();

  if age >= 18 && age < 60 {
      println!("Puedes entrar");
  }
  else {
      println!("No puedes entrar");
  }

  // Tarea
  let mut pill: String = String::new();

  println!("Píldora azul (a) o roja (r): ");
  std::io::stdin().read_line(&mut pill).unwrap();

  if pill.trim().to_string() == "a" || pill.trim().to_string() == "r"{
      println!("Buena decisión")
  }
  else {
      println!("la cagaste padrino")
  }
}