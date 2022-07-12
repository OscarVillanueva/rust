fn arrays() {

  let mut names: Vec<String> = Vec::new();
  
  for _ in 0..3 {
      
    println!("Por favor introduce un nombre");

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();

    names.push(name.trim().to_string());

  }

  for name in names {
    print!("{}", name)
  }

}
