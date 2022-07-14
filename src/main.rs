use csv::{ ReaderBuilder, StringRecord, Reader };
use std::{ fs };
use std::collections::{HashMap};

const FILENAME: &str = "assets/history.csv";
const FIRST_TAG: &str = "INICIO";

// TIPO, TAG, TEXT, +-VIDA;
#[derive(Debug)]
struct HistoryPart{
    data_type: String,
    life: i32,
    tag: String,
    text: String,
    options: Vec<HistoryPart>
}

impl HistoryPart {
    fn new(row: StringRecord) -> HistoryPart {
        HistoryPart {
            data_type: row.get(0).unwrap().trim().to_string(),
            life: row.get(3).unwrap().trim().parse::<i32>().unwrap_or(0),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            options: vec![]
        }
    }
}

fn prepare_history_parts(mut rdr: Reader<&[u8]>) -> HashMap<String, HistoryPart> {
    
    let mut last_record: String = "".to_string();
    let mut parts: HashMap<String, HistoryPart> = HashMap::new();

    for result in rdr.records() {

        let part = HistoryPart::new(result.unwrap());

        if part.data_type == "SITUACION" {

            last_record = part.tag.clone();
            parts.insert(part.tag.clone(), part);
        }
        else {
            if part.data_type == "OPCION" {

                // let data : Option<&mut HistoryPart> = history_parts.get_mut(&last_record);
                // match data {
                //     Some(data) => data.options.push(part),
                //     None => {continue;},
                // }

                // es como un guard let de swift
                // .get_mut puede devolver un None y verificamos que exista un valor
                if let Some(data) = parts.get_mut(&last_record) {

                    data.options.push(part)

                }

                
            }
        }


    }

    parts

}

fn main() {

    let mut life = 100;
    let mut current_tag = FIRST_TAG;

    // Leemos el contenido del archivo
    let content = fs::read_to_string(FILENAME).unwrap();

    // Lo pasamos a "tabla"
    let rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    // Generamos la historia
    let history_parts = prepare_history_parts(rdr);

    
    loop {
        
        if let Some(data) = history_parts.get(current_tag) {
            
            life = life + data.life;

            println!("\n");
            println!("Tienes {} de vida", life);
            println!("{}", data.text);

            if life <= 0 { 
                println!("\n\n");
                println!("======================");
                println!("||                 ||");
                println!("|| Juego terminado ||");
                println!("||                 ||");
                println!("======================");
                break; 
            }

            for (idx, option) in data.options.iter().enumerate() {
                println!("[{}] {}", idx,option.text)
            }

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();

            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(selected) = data.options.get(selection) {
                current_tag = &selected.tag;
            }
            else {
                println!("Comando no válido");
            }

        }
        else {
            println!("\n\n");
            println!("===========================================================");
            println!("|| Por el momento la historia ha terminado               ||");
            println!("|| Puedes imaginar que pasa después                      ||");
            println!("|| La historia aún continua                              ||");
            println!("===========================================================");
            break;
        }

    }
    
}
