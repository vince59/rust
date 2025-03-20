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

fn test_match2() {
    let jete_de_de = 9;
    match jete_de_de {
        // le match impose de tester toutes les valeurs
        3 => println!("trois"),
        7 => println!("sept"),
        autre_entier => println!("autre : {autre_entier}"), // soit cas par défaut, soit on teste toutes les valeur possibles
        _ => (), // inutile ici mais si ne veux pas récupérer la valeur par défaut on peut mettre le _ et pour ne rien faire on met ()
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
    // un tableau à une taille fixe, on ne peut pas ajouter d'élément dedans
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
    let mut x = [[0; 4], [0; 4]]; // tableau à plusieurs dimensions
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
            if y == 5 {
                break 'boucle1;
            }
            println!("boucle 2 y={y}");
            y += 1;
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

fn prendre_possetion(texte: String) {
    println!("{texte}");
}

fn prend_et_rend(texte: String) -> String {
    println!("{texte}");
    texte
}

fn calculer_taille(s: String) -> (String, usize) {
    let taille = s.len(); // len() retourne la taille d'une String.

    (s, taille) // retourne un tuple
}

fn test_possetion() {
    let s1 = String::from("hello");
    let s2 = s1; // s2 prend la possétion et s1 est détruit
    // println!("s1 = {}, s2 = {}", s1, s2); // plante à la compil car s1 est détruit

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // pour les scalaires ça passe car les données sont copiées dans la pile
    prendre_possetion(s2);
    // println!("{s2}"); // ça plante car l'appel à la fonction à détruit la chaine.
    let s1 = String::from("hello");
    let s2 = prend_et_rend(s1);
    println!("s2 = {s2}"); // ça passe la chaine initiale n'a pas été détruite

    let s1 = String::from("hello");
    let (s2, taille) = calculer_taille(s1);
    println!("La taille de '{}' est {}.", s2, taille);
}

fn calculer_taille2(s: &String) -> usize {
    s.len()
}

fn test_reference() {
    let s1 = String::from("hello");
    let long = calculer_taille2(&s1); // le & permet d'utiliser la variable sans en prendre possétion
    println!("La taille de '{}' est {}.", s1, long); // donc s1 est toujours utilisable après l'appel à la fonction
}

fn calculer_taille_et_modifie(s: &mut String) -> usize {
    s.push_str(" toto");
    s.len()
}

fn test_reference_mutable() {
    let mut s1 = String::from("hello");
    let long = calculer_taille_et_modifie(&mut s1); // le & permet d'utiliser la variable sans en prendre possétion et mut qu'on authorise sa modification
    println!("La taille de '{}' est {}.", s1, long); // donc s1 est toujours utilisable après l'appel à la fonction

    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s; // ne compile pas car on ne peut avoir qu'une seule référence mutable à un instant t
    //println!("{}, {}", r1, r2);
}

fn test_slice() {
    let mut a = [1, 2, 3, 4, 5];
    {
        let slice = &mut a[1..3]; // slice est un extrait du tableau
        // a[2]=10; // interdit car tant que la slice est dans la portée a ne peut plus être modifiée
        slice[0] = 99;
        println!("{slice:?}");
    }
    a[3] = 100;
    println!("{a:?}");
}

// déclaration
#[derive(Debug)] // permet l'impléùentation automatique del'affichage pour le debug
struct Utilisateur {
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
}

fn creer_utilisateur(email: String, pseudo: String) -> Utilisateur {
    Utilisateur {
        email,  // ou email: email,
        pseudo, // ou pseudo: pseudo,
        actif: true,
        nombre_de_connexions: 1,
    }
}

fn test_structure() {
    // instanciation
    let mut utilisateur1 = Utilisateur {
        email: String::from("quelquun@example.com"),
        pseudo: String::from("pseudoquelconque123"),
        actif: true,
        nombre_de_connexions: 1,
    };

    //accès aux champs
    utilisateur1.email = String::from("unautremail@example.com");

    // instanciation via une fonction
    let utilisateur2 = creer_utilisateur(
        String::from("quelquun@example.com"),
        String::from("pseudoquelconque123"),
    );

    let utilisateur3 = Utilisateur {
        email: String::from("quelquundautre@example.com"),
        ..utilisateur1 // ajoute les autres attributs mais utilisateur1.pseudo n'est pas accessible il faudra utiliser utilisateur3.pseudo
    };

    println!("utilisateur1.pseudo = {}", utilisateur1.email); // on ne peut pas mettre pseudo car u
    println!("utilisateur2.pseudo = {}", utilisateur2.pseudo);
    println!("utilisateur3.pseudo = {}", utilisateur3.pseudo);

    dbg!(utilisateur2);

    // on peut faire des structures sans nommer les champs un peu comme les tuples :
    struct Couleur(i32, i32, i32);
    struct Point(i32, i32, i32);
    let noir = Couleur(0, 0, 0);
    let mut origine = Point(0, 0, 0);
    origine.2 = 3;
    println!("origine.2 = {}", origine.2);
}

struct Rectangle {
    // champs de la classe
    hauteur: u32,
    largeur: u32,
}

impl Rectangle {
    // fonction de la classe
    fn aire(&self) -> u32 {
        // comme en python 1er paramètre = self
        self.hauteur * self.largeur
    }

    fn carre(cote: u32) -> Rectangle {
        // fonction statique qui peut être utilisé comme constructeur
        Rectangle {
            largeur: cote,
            hauteur: cote,
        }
    }
}

fn test_methode() {
    let rect1 = Rectangle {
        hauteur: 20,
        largeur: 30,
    };
    println!("aire = {}", rect1.aire());
    let carre = Rectangle::carre(10);
    println!("aire du carré = {}", carre.aire());
}

#[derive(Debug)]
enum Message {
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}

impl Message {
    // on ajoute des méthodes à l'enum, les méthodes sont communes à toutes les variantes de l'enum
    fn appeler(&self, p: &str) {
        println!("ici on va faire un truc {p}");
        dbg!(self);
    }
}

#[derive(Debug)] // pour pouvoir afficher l'État
enum EtatUs {
    // ex: liste des états américains
    Alabama,
    Alaska,
}

enum PieceUs {
    // type de pièces de monnaies US
    Penny,
    Nickel,
    Dime,
    Quarter(EtatUs), // pour cette pièce on stocke aussi l'état car la pièce est différente selon l'état
}

fn valeur_en_centimes(piece: PieceUs) -> u8 {
    match piece {
        // on peut utiliser match avec les énums
        PieceUs::Penny => 1,
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        // PieceUs::Quarter(_) => {println!("toto"); 10}, // ici on ne récupère pas le paramètre
        PieceUs::Quarter(etat) => {
            // le match va tenir compte du motif et on peut récupérrer la valeur du paramètre
            println!("Il s'agit d'un quarter de l'État de {:?} !", etat);
            25
        }
    }
}

fn test_enum() {
    let mess_quitter = Message::Quitter;
    let mess_deplacer = Message::Deplacer { x: 20, y: 30 }; // on instancie une des variantes de l'enum en passant les données
    mess_deplacer.appeler("bidule"); // on appel la méthode de l'énum
    let c = Message::ChangerCouleur(1, 2, 3);
    c.appeler("truc");
    mess_quitter.appeler("quitter");

    let piece = PieceUs::Quarter(EtatUs::Alabama); // on instancie une variante de Piece
    let centimes = valeur_en_centimes(piece); // on peut passer l'instance
    let centimes = valeur_en_centimes(PieceUs::Quarter(EtatUs::Alabama)); // ou la valeur directement
    println!("{centimes}");
}

fn plus_un(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn test_option() {
    let cinq = Some(5);
    let six = plus_un(cinq);
    let none = plus_un(None);
}

fn test_if_let() {
    let une_valeur_u8 = Some(0o77u8); // 77 en base octale stocké dans un unsigned int sur 8 bits
    if let Some(max) = une_valeur_u8 {
        println!("Le maximum est réglé sur {}", max);
    }

    // équivallent à :

    match une_valeur_u8 {
        Some(max) => println!("Le maximum est réglé sur {}", max),
        _ => (),
    }
}

fn test_vecteurs() {
    let v = vec![1, 2, 3, 4]; // déclaration d'un vecteur, tous les éléments d'un vecteur sont forcément du même type
    let troisieme: &i32 = &v[2]; // accés à un indice (commence à 0)
    println!("Le troisième élément est {}", troisieme);

    match v.get(2) {
        // on peut y accéder avec get et gérer le débordement d'indice
        Some(troisieme) => println!("Le troisième élément est {}", troisieme),
        None => println!("Il n'y a pas de troisième élément."),
    }

    //let existe_pas = &v[100]; // plante à l'éxécution
    let existe_pas = v.get(100);
    match existe_pas {
        Some(x) => println!("élément 100 est {}", x),
        None => println!("erreur"),
    }

    let mut vec = vec![1, 2, 3, 4];

    vec.push(5); // Ajoute un élément à la fin
    println!("vec1 = {:?}", vec); // 

    let tableau = [1, 2, 3, 4];
    let mut vec: Vec<i32> = tableau.to_vec(); // convertis un tableau en vecteurs

    vec.push(9);
    println!("vec2 = {:?}", vec);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // itération sur un vecteur et &mut indique que l'on va modifier le vecteur
        *i += 50; // le *i déréférence i, on peut alors changer la valeur du vecteur, ici on ajoute 50 à chaque occurrence
    }
    println!("vec2 = {:?}", v);

    for i in v {
        // itération sur un vecteur et &mut indique que l'on va modifier le vecteur
        println!("i = {}", i);
    }

    let vec2 = vec!["a", "b", "c"];

    for (index, value) in vec2.iter().enumerate() { // permet d'itérer sur le vecteur en ayant l'indice on peut utiliser iter_mut pour modifier le contenu
        println!("Indice: {}, Valeur: {}", index, value);
    }

    // on montre ici qu'avec une énumération on peut stocker des types différents (mais exhaustif et qu'on connait au départ) 
    // dans un vecteur en utilant un enum :
    #[derive(Debug)]
    enum Cellule {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let ligne = vec![
        Cellule::Int(3),
        Cellule::Text(String::from("bleu")),
        Cellule::Float(10.12),
    ];

    for val in ligne {
        println!("{:?}",val);
    }
}

fn test_string(){
    let mut s = String::new(); // Création d'une chaine vide

    let donnee = "contenu initial1";

    let s = donnee.to_string(); // création d'une chaine à partir d'un str

    // cette méthode fonctionne aussi directement sur un
    // littéral de chaîne de caractères :
    let s = "contenu initial2".to_string();
    //ou 
    let s = String::from("contenu initial3"); /// idem

    println!("{s}");

    let mut s = String::from("foo");
    s.push_str("bar"); // ajoute une chaine à la fin
    println!("{s}");

    let mut s = String::from("lo");
    s.push('l'); // on ajoute un seul caractère à la chaine
    println!("{s}");
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
    test_reference_mutable();
    test_slice();
    test_structure();
    test_methode();
    test_enum();
    test_option();
    test_match2();
    test_if_let();
    test_vecteurs();
    test_string();
}
