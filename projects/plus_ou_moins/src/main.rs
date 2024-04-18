use std::io;
use rand::Rng;
use std::cmp::Ordering;
// Less/greater/equal
fn main() {
    println!("Devinez le nombre!");

    let nombre_secret = rand::thread_rng().gen_range(1..101);
    println!("le nombre secret est {nombre_secret}");

    
    loop{
        let mut supposition = String::new();
        println!("veuillez entrer un nombre");
        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de le lecture de l'entree utilisateur");
    
        let supposition: u32 = match supposition.trim().parse(){
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        println!("votre nombre: {supposition}");
        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
