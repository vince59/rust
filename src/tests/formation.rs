use rand::Rng;
use std::cmp::Ordering;
use std::io; // crate (boîte / caisse) librairie externe (à ajouter dans le cargo.toml)

fn test_input_output() {
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
    // let supposition: u32 = "42a".parse().expect("Ce n'est pas un nombre !"); // plante à l'exécution
    let supposition: u32 = "42".parse().expect("Ce n'est pas un nombre !");
    println!("supposition = {supposition}");
}

fn test_type() {
    let x: i16 = 1_235; // Décimal
    let y: i16 = 0xff; // hexa
    let z: i16 = 0o77; // octal
    let a: i16 = 0b1111_0000; // binaire 
    let b: u8 = b'A'; // octet (code du caractère ascii)
    let c: u8 = 13;
    let t = true;
    let f: bool = false; // avec une annotation de type explicite
    let c1 = 'z';
    let z1 = 'ℤ';
    let chat_aux_yeux_de_coeur = '😻';
    println!("x = {x}, y = {y}, z = {z}, a = {a}, b = {b}, c = {c}, t = {t}, f = {f}");
    println!("chat_aux_yeux_de_coeur = {chat_aux_yeux_de_coeur}");
}

fn test_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let y1 = tup.1;
    let valeur_unité = ();
    println!("x = {x}, y1 = {y1}, tup = {tup:?}");
}

fn test_tableau() {
    let mois = [
        "Janvier",
        "Février",
        "Mars",
        "Avril",
        "Mai",
        "Juin",
        "Juillet",
        "Août",
        "Septembre",
        "Octobre",
        "Novembre",
        "Décembre",
    ];
    println!("mois : {}", mois[0]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {}", a[0]);
    let a = [3; 5]; // initialise un tableau de 5 élément contenant des 3
    println!("a = {a:?}");
    let mut x = [[0; 4], [0; 4]];
    x[0][0] = 10;
    x[0][1] = 20;
    x[0][2] = 30;
    x[0][3] = 40;
    x[1][0] = 50;
    x[1][1] = 60;
    x[1][2] = 70;
    x[1][3] = 80;
    println!("{x:?}");
}

fn zz(x: i32) -> i32 {
    3 * x // équivallent à return 3 * x
}

fn test_expression() {
    let y = {
        let x = 3;
        x + 1
    };
    let x = zz(2);
    println!("La valeur de y,y est : {y},{x}");
}

fn test_condition() {
    let nombre = 6;
    if nombre % 4 == 0 {
        println!("Le nombre est divisible par 4");
    } else if nombre % 3 == 0 {
        println!("Le nombre est divisible par 3");
    } else if nombre % 2 == 0 {
        println!("Le nombre est divisible par 2");
    } else {
        println!("Le nombre n'est pas divisible par 4, 3 ou 2");
    }
    let condition = true;
    let nombre = if condition { 5 } else { 6 };
    println!("La valeur du nombre est : {}", nombre);
}

fn test_boucle() {
    let mut y = 0;
    // boucle avec label d'identification, utilisé pour le break
    'boucle1: loop {
        println!("boucle1");
        loop {
            if y==5 {
                break 'boucle1;
            }
            println!("boucle 2 y={y}");
            y+=1;
        }
    }

    // boucle avec retour de valeur

    let mut compteur = 0;

    let resultat = loop {
        compteur += 1;

        if compteur == 10 {
            break compteur * 2; // une boucle peut renvoyer une valeur
        }
    };

    println!("Le résultat est {}", resultat);

    // boucle for sur tableau

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("La valeur est : {}", element);
    }

    // boucle sur interval

    for nombre in (1..=4).rev() {
        println!("{} !", nombre);
    }
    println!("DÉCOLLAGE !!!");

}

fn prendre_possetion(texte: String){
    println!("{texte}");
}

fn prend_et_rend(texte: String)->String{
    println!("{texte}");
    texte
}

fn calculer_taille(s: String) -> (String, usize) {
    let taille = s.len(); // len() retourne la taille d'une String.

    (s, taille) // retourne un tuple
}

fn test_possetion(){
    let s1 = String::from("hello");
    let s2 = s1; // s2 prend la possétion et s1 est détruit
    // println!("s1 = {}, s2 = {}", s1, s2); // plante à la compil car s1 est détruit

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // pour les scalaires ça passe car les données sont copiées dans la pile
    prendre_possetion(s2);
    // println!("{s2}"); // ça plante car l'appel à la fonction à détruit la chaine.
    let s1 = String::from("hello");
    let s2=prend_et_rend(s1); 
    println!("s2 = {s2}"); // ça passe la chaine initiale n'a pas été détruite

    let s1 = String::from("hello");
    let (s2, taille) = calculer_taille(s1);
    println!("La taille de '{}' est {}.", s2, taille);
}

fn calculer_taille2(s: &String) -> usize {
    s.len()
}

fn test_reference(){
    let s1 = String::from("hello");
    let long = calculer_taille2(&s1); // le & permet d'utiliser la variable sans en prendre possétion
    println!("La taille de '{}' est {}.", s1, long);
}

fn main() {
    test_condition();
    test_expression();
    test_tableau();
    test_tuple(); // test des tuples
    test_type(); // test type de données
    test_masquage(); // test masquage des variables
    //test_input_output(); // test entrée sortie standard + nombre aléatoire et interval
    //test_match(); // test du match (switch) et conversion de type chaine nombre boucle et break
    test_mutable(); // test mutabilité des variables, constante
    test_boucle();
    test_possetion();
    test_reference();
}
