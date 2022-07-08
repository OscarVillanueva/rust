use regex::Regex;

fn get_result(regex: &Regex, mut expression: String, operator: &&str) -> String {

    loop {

        // Aplicar Operaciones
        let caps = regex.captures(expression.as_str());

        if caps.is_none() { break; }

        let caps = caps.unwrap();

        let caps_expression = caps.get(0).unwrap().as_str();

        // Sacamos lo que este en la posición 1
        let left_value: f32 = caps.get(1).unwrap().as_str().parse::<f32>().unwrap();
        let right_value: f32 = caps.get(2).unwrap().as_str().parse::<f32>().unwrap();

        // &* entiendo que le decimos a rust que en la referencia de memoria pasa por operator
        // Debe matchar con * o sus opciones
        let result = match operator {
            &"*" => left_value * right_value,
            &"/" => left_value / right_value,
            &"+" => left_value + right_value,
            &"-" => left_value - right_value,
            _ => 0.0
        };

        expression = expression.replace(caps_expression, &result.to_string());
    }

    expression

}

fn main() {

    // https://stackoverflow.com/questions/36362020/what-is-unwrap-in-rust-and-what-is-it-used-for
    // En rust digamos que todos los tipos de datos están wrappeados en un genérico llamado
    // Option<T> por lo tanto usamos un unwrap para que nos devuelva el valor que contiene el
    // Genérico y no todo como algo Option<String>, Option<i32> si no simplemente el string 
    // o el número
    
    // En Rust los errores son un tipo de dato el tipo sera Result<T, E>, siendo T el tipo de 
    // error y E el error dado

    // Si se encuentra un error unwrap nos devuelve el error (mensaje) en caso de error

    // Regex 
    // Controlar las sumas
    // Match: 1 o más número, un espacio opcional +, un espacio opcional y 1 o más dígitos
    
    let regex = [
        (Regex::new(r"(\d+\.?\d?+)\s?\*\s?(\d+\.?\d?+)").unwrap(), "*"),
        (Regex::new(r"(\d+\.?\d?+)\s?/\s?(\d+\.?\d?+)").unwrap(), "/"),
        (Regex::new(r"(\d+\.?\d?+)\s?\+\s?(\d+\.?\d?+)").unwrap(), "+"),
        (Regex::new(r"(\d+\.?\d?+)\s?\-\s?(\d+\.?\d?+)").unwrap(), "-"),
    ];

    // Traer los datos del usuario
    println!("Por favor introduce tu expresión: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    for op in &regex {

        let (pattern, operator) = op;
        expression = get_result(pattern, expression, operator);

    }

    // Le puse el &regex porque si no el ownership cambiaría es decir que ahora el for sería
    // el dueño del arreglo de regex y no lo podría usar en las siguientes dos
    // desestructuraciones
    // Entonces ahora la función solo recibe la referencia donde están localizados, dichos
    // parámetros
    // let (_, t) = regex[0];
    // let (_, y) = regex[0];

    // Mostrar resultado
    println!("Resultado: {}", expression)

}
