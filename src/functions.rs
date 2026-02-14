use std::{fs, io};
use std::io::Write;
#[derive(Debug)]
pub struct Event {
    pub nom: String,
    pub date: String,
    pub description: String,
    pub accomplished: bool,
}

impl Event {
    pub fn new(nom: String, date: String, description: String, accomplished: bool) -> Self {
        Self { nom, date, description, accomplished }
    }

    pub fn to_line(&self) -> String {
        // Utilisation de format! comme tu l'as fait, c'est parfait.
        format!("{}|{}|{}|{}", self.nom, self.date, self.description, self.accomplished)
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('|').collect();

        // On vérifie la taille et on tente de parser le booléen proprement
        if parts.len() == 4 {
            let accomplished = parts[3].parse::<bool>().ok()?; // .ok()? renvoie None si le parse échoue

            Some(Self {
                nom: parts[0].to_string(),
                date: parts[1].to_string(),
                description: parts[2].to_string(),
                accomplished,
            })
        } else {
            None
        }
    }
}
pub fn read_i8() -> i8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Échec de la lecture");

    // On essaie de parser, si ça rate on renvoie une valeur invalide (-1)
    // qui sera gérée par ton secure_choice
    input.trim().parse::<i8>().unwrap_or(-1)
}

pub fn secure_choice(start : i8, end : i8, choice : i8) -> i8 {
    let mut ch : i8 = choice;
    while ch < start || ch > end {
        println!("Invalid choice. Please try again.");
        println!("Choississez une valeur de {} a {}", start, end);
        print!("Choice : ");
        io::stdout().flush().unwrap();
        ch = read_i8();
    }
    ch
}

pub fn read_name() -> String {
    let mut input : String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}