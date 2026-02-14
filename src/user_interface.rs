use std::io;
use std::io::Write;
use crate::functions;
use crate::functions::{read_name, Event};
use crate::storage;

fn menu_1() -> i8 {
    // Le r#"..."# permet d'écrire sur plusieurs lignes naturellement
    print!(r#"
1. Voir les programmes a venir
2. Ajouter un programme
3. Supprimer un programme
4. Marquer un programme comme accompli
5. Quitter"#);

    // On force l'affichage car print! (sans \n) attend normalement que le tampon soit plein
    io::stdout().flush().unwrap();
    print!("\nVotre choix: ");
    io::stdout().flush().unwrap();
    let mut choice = functions::read_i8();
    if choice < 1 || choice > 5 {
        choice = functions::secure_choice(1,5,choice);
    }
    choice
}

fn invite_commande(sentences : &str) {
    print!("{} : ", sentences);
    io::stdout().flush().unwrap();
}

fn add() {
    println!("Nous allons ajouter un nouvel evenement a vos programmes");
    invite_commande("Nom de l'evenement");
    let name : String = read_name();
    invite_commande("La date de l'evenement");
    let date : String = read_name();
    invite_commande("Description");
    let description : String = read_name();
    let event : Event = Event::new(name, date, description, false);
    storage::add_event(&event).expect("Something went wrong");
    println!("L'evenement a ete ajoute avec success");
}

fn delete() {
    invite_commande("Le nom de l'evenement a supprimer");
    let name : String = read_name();
    storage::remove_event(&name).expect("Something went wrong");
}

fn achieve(){
    invite_commande("Le nom de l'evenement a achever");
    let name : String = read_name();
    storage::complete_event(&name).expect("Something went wrong");
}
pub fn run_application() {
    loop {
        let choice : i8 = menu_1();
        match choice {
            1 => storage::see_all_events().expect("Something went wrong"),
            2 => add(),
            3 => delete(),
            4 => achieve(),
            5 => {
                println!("Aurevoir a la prochaine planification");
                break;
            },
            _ => println!("Une erreur est survenue\n"),
        }
    }

}