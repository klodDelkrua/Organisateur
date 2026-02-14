use std::io::{self, Write, BufRead, BufReader};
use std::fs::OpenOptions;
use crate::functions::{Event};
use std::fs::File;
use std::fs;
pub fn add_event(event: &Event) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open("event.txt")?;
    writeln!(file, "{}", event.to_line())?;
    Ok(())
}

pub fn see_all_events() -> io::Result<()> {
    let file = File::open("event.txt")?;
    let reader = BufReader::new(file);

    println!("\n--- LISTE DES ÉVÉNEMENTS ---");
    for line in reader.lines() {
        let line = line?;
        if let Some(event) = Event::from_line(&line) {
            let status = if event.accomplished { "[X]" } else { "[ ]" };
            println!("{} {} - {} ({})", status, event.nom, event.description, event.date);
        }
    }
    println!("----------------------------\n");
    Ok(())
}

pub fn remove_event(event_name: &str) -> io::Result<()> {
    let filename = "event.txt";

    // 1. Lire le fichier (gérer le cas où le fichier n'existe pas encore)
    let content = fs::read_to_string(filename)?;

    // 2. Filtrer les lignes
    let filtered: Vec<String> = content
        .lines()
        .filter(|line| {
            // On ne garde la ligne que si elle ne commence PAS par "nom|"
            // On ajoute le '|' pour éviter de supprimer "Sport" si on cherche "Sp"
            !line.starts_with(&format!("{}|", event_name))
        })
        .map(String::from)
        .collect();

    // 3. Réécrire le fichier
    // Au lieu de : filtered.join("\n")
    // Fais ceci :
    let mut final_content = filtered.join("\n");
    if !final_content.is_empty() {
        final_content.push('\n'); // On s'assure que le fichier finit par une ligne vide
    }
    fs::write(filename, final_content)?;

    Ok(())
}

pub fn complete_event(event_name: &str) -> io::Result<()> {
    let filename = "event.txt";
    let content = fs::read_to_string(filename)?;

    let prefix = format!("{}|", event_name);

    let updated: Vec<String> = content
        .lines()
        .map(|line| {
            if line.starts_with(&prefix) {
                // 1. On transforme la ligne en structure Event
                if let Some(mut event) = Event::from_line(line) {
                    // 2. On modifie la variable
                    event.accomplished = true;
                    // 3. On redonne la version String modifiée
                    return event.to_line();
                }
            }
            // Si ce n'est pas l'événement recherché, on garde la ligne intacte
            line.to_string()
        })
        .collect();

    //fs::write(filename, updated.join("\n"))?;
    // Au lieu de : filtered.join("\n")
    // Fais ceci :
    let mut final_content = updated.join("\n");
    if !final_content.is_empty() {
        final_content.push('\n'); // On s'assure que le fichier finit par une ligne vide
    }
    fs::write(filename, final_content)?;
    Ok(())
}