fn loops() {

  // \ para poder hacer string de más linea pero sin salto de linea en consola
  // \n para salto de linea
  
  // Dos números que vamos a sumar
  let num_1 = 123;
  let num_2 = 321;

  let add = num_1 + num_2;

  loop {

      // Mostrar los dos números en pantalla
      println!("Por favor escribir la suma de {} y {}", num_1, num_2);
  
      // Obtener del usuario el número que representa la suma
      let mut user_sum = String::new();
  
      std::io::stdin().read_line(&mut user_sum).unwrap();
  
      let user_sum_int: i32 = user_sum.trim().parse().unwrap();
  
      if user_sum_int == add {
          println!("Lo has hecho muy bien, el resultado es {} es correcto", user_sum_int);
          break;
      }
      else {
          println!("El resultado no es correcto por favor intenta de nuevo");
      }

  }


}
