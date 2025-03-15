use rand::Rng;
use std::cmp::Ordering;
use std::io; // crate (boîte / caisse) librairie externe (à ajouter dans le cargo.toml)

fn input_output() {
    //let nombre_secret =rand::rng().random_range(1..101); // de 1 à 100
    let nombre_secret = rand::rng().random_range(1..=100); // de 1 à 100
    println!("Devinez le nombre {}", nombre_secret);
    println!("Veuillez entrer un nombre");

    let mut supposition = String::new(); // mut indique que la variable peut être modifiée

    io::stdin()
        .read_line(&mut supposition) // la variable est passée par adresse et la saisie utilisateur viens s'ajouter à la chaine de départ
        .expect("Échec de la lecture de l'entrée utilisateur");

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    println!("Votre nombre : {}", supposition);
}

fn test_match() {
    let nombre_secret = rand::rng().random_range(1..=100); // de 1 à 100
    println!("Devinez le nombre ! {}", nombre_secret);

    loop {
        let mut supposition = String::new();
        println!("Veuillez entrer un nombre.");

        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let supposition: u32 = supposition
            .trim()
            .parse()
            .expect("Veuillez entrer un nombre !");

        println!("Votre nombre : [{}]", supposition);

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

fn test_mutable() {
    const A: i16 = 10; // on doit préciser le type et c'est forcément une constante, on met le nom en majuscule (convention)
    let nombre_secret = rand::rng().random_range(1..=100);

    // let déclare la variable dans le scope, on peut redéclarer la même variable ++ fois
    let x = 5 + nombre_secret; // la variable ne peut pas être modifiée mais est initialisée avec une expression
    println!("La valeur de nombre_secret,x est : {} {}", nombre_secret, x);
    //let x = 6; //: plante !
    let mut y = 7; // la variable peut être modifiée
    y = y + x + 5 + A;
    println!("La valeur de x,A,y est : {} {} {}", x, A, y);
}

fn test_masquage() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("La valeur de x dans la portée interne est : {}", x);
    }
    println!("La valeur de x est : {}", x);
    /*
    let mut espaces = "   ";
    espaces = espaces.len(); // plante car on ne peut pas changer le type de la variable sans la redéclarer
     */
    let mut espaces = String::from("   ");
    espaces.push_str("toto");
    let espaces = espaces.len(); // ok car on redéclare espace
    println!("La longueur de espaces est : {espaces}");
}

fn 

fn main() {
    test_masquage(); // test masquage des variables
    return;
    input_output(); // test entrée sortie standard + nombre aléatoire et interval
    test_match(); // test du match (switch) et conversion de type chaine nombre boucle et break
    test_mutable(); // test mutabilité des variables, constante    
}
