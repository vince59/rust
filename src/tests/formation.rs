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
    let tab: [i32; 10] = std::array::from_fn(|i| (i + 1) as i32);
    println!("truc = {:?}",&tab[3..7]); // permet de récupérer les indices de 3 à 7
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

    for (index, value) in vec2.iter().enumerate() {
        // permet d'itérer sur le vecteur en ayant l'indice on peut utiliser iter_mut pour modifier le contenu
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
        println!("{:?}", val);
    }
}

fn test_string() {
    let mut s = String::new(); // Création d'une chaine vide

    let donnee = "contenu initial1";

    let s = donnee.to_string(); // création d'une chaine à partir d'un str

    // cette méthode fonctionne aussi directement sur un
    // littéral de chaîne de caractères :
    let s = "contenu initial2".to_string();
    //ou
    let s = String::from("contenu initial3");
    /// idem

    println!("{s}");

    let mut s = String::from("foo");
    s.push_str("bar"); // ajoute une chaine à la fin
    println!("{s}");

    let mut s = String::from("lo");
    s.push('l'); // on ajoute un seul caractère à la chaine
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // notez que s1 a été déplacé dans s3 et ne pourra plus être utilisé (s1 est détruit)
    println!("s2 = {s2}");
    println!("s3 = {s3}");

    // si on veut concaténer plusieurs chaine il vaut mieux utiliser la macro format! :

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // ne prend pas possession des variables
    println!("s={s}");
    println!("{}-{}-{}", s1, s2, s3);

    for c in "ça va ?नमस्ते".chars() {
        // itérer sur les caractères de la chaine (utf8)
        println!("{}", c);
    }
    let s = 24589.to_string(); // Tous les types qui implémente Display implémente aussi ToString et donc la méthode to_string
    println!("{s}");
}

fn test_table_de_hachage() {
    use std::collections::HashMap;

    let mut scores = HashMap::new(); // équivallent des tableaux associatif du php

    scores.insert(String::from("Bleu"), 10); // équivallent en json : {"Bleu":10,"Jaune",50}
    scores.insert(String::from("Jaune"), 50);

    let nom_equipe = String::from("Bleu");
    let score = scores.get(&nom_equipe);

    println!("score = {:?}", score);
    println!("score = {:?}", scores.get("Rouge")); // va donner None

    // itérer sur la table de hachage
    for (cle, valeur) in &scores {
        println!("clé = {} : valeur = {}", cle, valeur);
    }

    // construire une table de hachage avec un vecteur contenant les clé et un autre les valeurs :

    let equipes = vec![String::from("Bleu"), String::from("Jaune")]; // les clés
    let scores_initiaux = vec![10, 50]; // les valeurs

    let mut scores: HashMap<_, _> = equipes
        .into_iter()
        .zip(scores_initiaux.into_iter())
        .collect();
    println!("{:?}", scores);

    let nom_champ = String::from("Couleur favorite");
    let valeur_champ = String::from("Bleu");

    let mut table = HashMap::new();
    table.insert(nom_champ, valeur_champ);
    println!("{:?}", table);

    //println!("{}",nom_champ); // ici on n peut pas faire ça car nom_champ n'existe plus

    // changer la valeur d'une clé :
    scores.insert(String::from("Bleu"), 10);
    scores.insert(String::from("Bleu"), 25); // le 25 écrase le 10

    // Ajoute un valeur que si la clé n'existe pas déjà :

    scores.entry(String::from("Jaune")).or_insert(50); // Jaune = 50
    scores.entry(String::from("Bleu")).or_insert(50); // on garde le bleu = 25

    println!("{:?}", scores);

    // Modifier une valeur en fonction de l'ancienne :

    let texte = "bonjour le monde magnifique monde";

    let mut table = HashMap::new();

    for mot in texte.split_whitespace() {
        // boucle sur les mots si il y a un espace entre chauqe
        let compteur = table.entry(mot).or_insert(0); // retourne l'adresse de la valeur de la clé
        *compteur += 1; // on ajoute 1 à la valeur du coup ça compte le nombre de fois qu'on a le même mot
    }
    println!("nb mots = {:?}", table);

    let mut th: HashMap<_, _> = HashMap::new(); // la clé et la valeur peuvent être ce qu'on veut mais identique pour toutes les occurences
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    th.insert("clé", a); // ici la valeur c'est un tableau
    println!("valeurs = {:?}", th);

    let mut th2: HashMap<_, _> = HashMap::new();

    #[derive(Debug)]
    // la valeur peut être un énum
    enum Cellule {
        Int(i32),
        Float(f64),
        Text(String),
        Toto([i32; 5]),
    }

    th2.insert("clé 1", Cellule::Float((10.2))); // ici la valeur c'est un flotant
    th2.insert("clé 2", Cellule::Int(10)); // ici la valeur c'est un entier de 10
    th2.insert("clé 3", Cellule::Text("toto".to_string())); // ici la valeur c'est une chaine
    th2.insert("clé 4", Cellule::Toto([10, 11, 12, 13, 55])); // ici la valeur c'est un tableau de 5 entiers

    println!("{:?}", th2);
}

fn test_panic() {
    //panic!("Erreur fin du programme"); // déclenche l'arrête du programme et affiche l'erreur

    let v = vec![1, 2, 3];
    //v[99]; // provoque un panic car l'indice n'existe pas
    // pour avoir la back trace :
    //$env:RUST_BACKTRACE=1; cargo run
}

// le result ne termine pas le programme et renvoi la gestion de l'erreur au programme appelant
fn test_result() {
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("hello.txt"); // renvoi un Result, pas le gestionnaire lui-même
    let f = match f {
        // ici f va recevoir le gestionnaire de fichier sinon le programme plante
        Ok(fichier) => fichier,
        Err(erreur) => panic!("Erreur d'ouverture du fichier : {:?}", erreur),
    };

    // là on fait pareil mais on test les différents types d'erreur
    let f = File::open("hello.txt");
    let f = match f {
        Ok(fichier) => fichier, // le fichier est ouvert, on le renvoi
        Err(erreur) => match erreur.kind() {
            // ça n'a pas marché, on regarde pourquoi
            ErrorKind::NotFound => match File::create("hello.txt") {
                // il n'existe pas => on le cree
                Ok(fc) => fc, // si la création est ok on retourne le gestionnaire de fichier
                Err(e) => panic!("Erreur de création du fichier : {:?}", e), // la création peut aussi planter
            },
            autre_erreur => {
                // si c'est un autre problème on s'arrête et on affiche l'erreur
                panic!("Erreur d'ouverture du fichier : {:?}", autre_erreur)
            }
        },
    };

    // on peut utiliser unwrap_or_else pour éviter d'utiliser des match imbriqués :
    let f = File::open("hello.txt").unwrap_or_else(|erreur| {
        if erreur.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|erreur| {
                panic!("Erreur de création du fichier : {:?}", erreur);
            })
        } else {
            panic!("Erreur d'ouverture du fichier : {:?}", erreur);
        }
    });

    // encore plus concis :
    // Si la valeur de Result est la variante Ok, unwrap va retourner la valeur contenue dans le Ok.
    // Si le Result est la variante Err, unwrap va appeler la macro panic!
    let f = File::open("hello.txt").unwrap();

    // on peut aussi faire la même chose mais améliorer le message d'erreur
    // à l'éxécution on aura le message d'erreur prévu par le compilateur et le notre :
    let f = File::open("hello.txt").expect("Échec à l'ouverture de hello.txt");
}

use std::fs::File;
use std::io::Read;

// test de propagation des erreurs
// la fonction renvoi un result donc Ok ici une chaine ou Err ici la structure error du module
fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(fichier) => fichier,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        // ici on va renvoyer une des options Ok ou Err (pas la chaine directement)
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } // on ne met pas le ; pour renvoyer la valeur
}

fn test_result_propagation() {
    let pseudo = lire_pseudo_depuis_fichier().unwrap();
}

fn lire_pseudo_depuis_fichier2() -> Result<String, io::Error> {
    // si il n'y a pas d'erreur on récupère le fichier dans f
    // si il y a une erreur on quitte la fonction et on retourne l'erreur
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    // là on lit le fichier si il y a une erreur on quitte la fonction avec l'erreur
    // sinon on a la chaine lue dans s
    f.read_to_string(&mut s)?;
    Ok(s) // on quitte la fonction avec en renvoyant l'option Ok avec sa valeur qui est la chaine lue
}

// et encore plus concis :

fn lire_pseudo_depuis_fichier3() -> Result<String, io::Error> {
    let mut s = String::new();
    // on peut tout mettre sur une seule ligne
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

use std::fs;

fn lire_pseudo_depuis_fichier4(s: &str) -> Result<String, io::Error> {
    fs::read_to_string(s) // lis le contenu du fichier et renvoi soit la chaine soit l'erreur
}

fn dernier_caractere_de_la_premiere_ligne(texte: &str) -> Option<char> {
    // texte.lines() : crée un itérateur sur les lignes du fichier
    // .next() : renvoi soit la premiere ligne soit None
    // ? : Renvoi None si next() renvoi None et termine la fonction, ou renvoi la chaine
    // .chars() retourne un intérateur sur les caractères de la chaine
    // .last() retourne le dernier caratère
    texte.lines().next()?.chars().last()
}

fn test_result_propagation_avec_point_interrogation() {
    //let pseudo = lire_pseudo_depuis_fichier2().unwrap();
    //let pseudo = lire_pseudo_depuis_fichier3().unwrap();

    let pseudo = lire_pseudo_depuis_fichier4("C:\\rust\\rust\\src\\tests\\pseudo.txt").unwrap();
    println!("{pseudo}");
    println!("{:?}", dernier_caractere_de_la_premiere_ligne(&pseudo));
}

#[derive(Debug)]
struct Point<T> {
    // structure générique <T> indique que la structure varie en fonction de T, T c'est le nom du type générique
    x: T, // on deux attributs x et y de n'importe quel type (mais du même type ...)
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    // structure générique <T,U> indique que la structure varie en fonction de T et de U
    x: T, // on deux attributs x et y qui peuvent être de type différents
    y: U,
}

fn test_structure_generique() {
    let entiers = Point { x: 5, y: 10 }; // on crée une instance de la structure avec des entiers 
    let flottants = Point { x: 1.0, y: 4.0 }; // ou des flotants par exemple

    println!("{:?}", flottants);

    let mixe1 = Point2 { x: 5, y: 'a' }; // on crée une instance de la structure avec un mixe de valeurs
    let mixe2 = Point2 { x: 7, y: 4.2 };

    println!("{:?}", mixe1);
}

impl<T> Point<T> {
    // on implémente la structure Point en y ajoutant la fonction qui retourne l'attribut x
    // la fonction x est disponible dans toutes les instances de Point
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    // on implémente la fonction distance_de_lorigine mais que pour le type f32
    // si l'instance de Point n'est pas de type f32 cette fonction n'est pas disponible :
    fn distance_depuis_lorigine(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn test_fonction_generique_structure() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    //println!("p.x = {}", p.distance_depuis_lorigine()); // Plante car pas de type f32

    let p = Point { x: 5.2, y: 10.8 };
    println!("p.x = {}", p.x());
    println!("p.x = {}", p.distance_depuis_lorigine());
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    // on implémente Point3 en fonction des types génériques X1 et Y1
    // on ajoute la fonction générique melange qui dépend des types génériques X2 et Y2
    // elle prend en paramètre une instance de la stucture Point3 construite avec les types X2 et Y2
    // (on est pas obligé de leur donner le même nom que dans la définition de la structure)
    // la fonction melange retourne une instance de Point3 avce les types génériques X2 et Y2
    fn melange<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            // ici on retourne une instance de Point3
            x: self.x,
            y: other.y,
        }
    }
}

fn test_fonction_generique_structure2() {
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.melange(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

struct Personne {
    nom: String,
    age: i32,
}
trait PeutCommuniquer {
    // nom de l'interface (un adjectif par exemple) c'est une caractéristique
    fn parler(&self) -> String; // Prototype qui devra être implémenté

    fn chanter(&self) -> String {
        // une autre fonction à implémenter mais avec un comportement par défaut
        String::from("la la la ...")
    }
}

impl PeutCommuniquer for Personne {
    // On implémente le trait PeutCommuniquer pour la structure Personne
    fn parler(&self) -> String {
        format!("Bonjour je suis {} et j'ai {} ans", self.nom, self.age)
    }
}

fn communiquer(individu: &impl PeutCommuniquer) {
    // on attend un paramètre on sait juste qu'il implémente le trait PeutCommuniquer
    println!(
        "je te parle mais je ne te connais pas ! {}",
        individu.parler()
    );
    //individu.nom="toto"; // on ne peut pas faire ça car on ne connais que les fonction du trait
}

// Cette syntaxe est équivallente mais plus verbeuse
fn communiquer2<T: PeutCommuniquer>(individu: &T) {
    println!(
        "je te parle mais je ne te connais pas ! {}",
        individu.parler()
    );
}

// on peut imposer que les deux paramètres de la fonction implémente chacun le trait mais avec cette syntaxe
// les deux paramètres peuvent être de type différents :

fn communiquer_avec(individu1: &impl PeutCommuniquer, individu2: &impl PeutCommuniquer) {
    println!(
        "ils parlent ensemble : {}, {}",
        individu1.parler(),
        individu2.parler()
    );
}

// si on veux les forcer à être du même type il faut alors utiliser cette syntaxe :

fn communiquer_avec2<T: PeutCommuniquer>(individu1: &T, individu2: &T) {
    println!(
        "ils parlent ensemble : \n{}, \n{}",
        individu1.parler(),
        individu2.parler()
    );
}

trait PeutBouger {
    fn avancer(&self) -> String; // Prototype qui devra être implémenté
}

impl PeutBouger for Personne {
    // On implémente le trait PeutBouger pour la structure Personne
    fn avancer(&self) -> String {
        String::from("Je bouge !")
    }
}

// une fonction peut demander à ce que les types implémentent plusieurs traits

fn parler_en_marchant(individu: &(impl PeutBouger + PeutCommuniquer)) {
    println!(
        "Je parle ! {} et en même temps {}",
        individu.parler(),
        individu.avancer()
    );
}

// la même avec des traits lié sur des types génériques

fn parler_en_marchant2<T: PeutBouger + PeutCommuniquer>(individu: &T) {
    println!(
        "Je parle ! {} et en même temps {}",
        individu.parler(),
        individu.avancer()
    );
}

// la même avec la syntaxe where

fn parler_en_marchant_avec<T, U>(individu1: &T, individu2: &U)
where
    T: PeutBouger + PeutCommuniquer,
    U: PeutBouger + PeutCommuniquer,
{
    println!(
        "On parle! {}\n{}\n et en même temps on {}\n{}",
        individu1.parler(),
        individu2.parler(),
        individu1.avancer(),
        individu2.avancer()
    );
}

// cette fonction retourne un objet d'un type que le programme appelalnt ne connaitra pas
// par contre il saura qu'il implémente le trait PeutCommuniquer
// c'est souvent utiliser pour les itérateurs

fn retourne_un_truc_qui_parle() -> impl PeutCommuniquer {
    Personne {
        nom: String::from("Bob"),
        age: 45,
    }
}

use std::env;

fn test_trait() {
    let p = Personne {
        nom: String::from("Vincent"),
        age: 10,
    };

    let p2 = Personne {
        nom: String::from("Toto"),
        age: 30,
    };

    println!("{}", p.parler());
    println!("{}", p.chanter());
    communiquer(&p);
    communiquer2(&p);
    communiquer_avec(&p, &p2);
    communiquer_avec2(&p, &p2);
    parler_en_marchant(&p);
    parler_en_marchant2(&p);
    parler_en_marchant_avec(&p, &p2);
}

// Récupération des arguments de la ligne de commande
fn test_argument_ligne_de_commande() {
    let args: Vec<String> = env::args().collect(); // Dans un vecteur
    println!("{:?}", args);
    let executable = &args[0];
    println!("Nom exécutable {:?}", executable);
}

fn test_lecture_fichier_texte() {
    let nom_fichier = "C:\\rust\\rust\\src\\tests\\pseudo.txt";
    let contenu = fs::read_to_string(nom_fichier)
        .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

    println!("Dans le texte :\n{}", contenu);
}

fn retour_2_entier() -> (i16, i16) {
    (10, 20)
}
fn test_retour_multiple() {
    let (x, y) = retour_2_entier();
    println!("x={x},y={y}");
}

fn un_sur(i: i16) -> Result<f32, &'static str> {
    if i == 0 {
        return Err("Division par 0 !");
    }
    //let r : f32 = ;
    Ok(1.0 / f32::from(i))
}

use std::process;

fn test_erreur() {
    let z = un_sur(3).unwrap_or_else(|err| {
        println!("Erreur : {}", err);
        process::exit(1);
    });
    println!("Z={z}");
}

use std::thread;
use std::time::Duration;

fn test_sleep() {
    //thread::sleep(Duration::from_secs(2)); // on attend deux sec
}

fn test_fermeture_lambda() {
    fn ajouter_un_v1(x: u32) -> u32 {
        // frontion qui ajoute 1
        x + 1
    }
    let ajouter_un_v2 = |x: u32| -> u32 { x + 1 }; // une fermeture (lambda ou fonction anonyme) avec déclaration des types
    let ajouter_un_v3 = |x| x + 1; // le type de x dépend de l'usage de la fermeture mais une fois déterminé on ne peut pas le changer
    let ajouter_un_v4 = |x| x + 1.0;
    ajouter_un_v3(1);
    ajouter_un_v4(1.0);
    let fermeture_exemple = |x| x;

    let s = fermeture_exemple(String::from("hello"));
    // let n = fermeture_exemple(5); ne compile pas car le paramètre x est une chaine suite au 1er appel

    // les fermetures ont accès aux variables de la portée :

    let x: i32 = 4;

    let egal_a_x = |z| {
        z == x // la fermeture à accès à x qui ne fait pas parti de la fonction
    };

    let y = 4;

    println!("x({})=y({}) ? : {}", x, y, egal_a_x(y));
    let x = 5; // Même si on redéfini x x garde la valeur initiale 4
    println!("x({})=y({}) ? : {}", x, y, egal_a_x(y));

    let x = vec![1, 2, 3];
    let egal_a_x = move |z| z == x; // avec le mot clé move la lambda détruit les variables de la portée qu'elle utilise 
    //println!("On ne peut pas utiliser x ici : {:?}", x); // x a été détruit dans la fermeture
    let y = vec![1, 2, 3];
    println!("---->{}", egal_a_x(y));
}

// utilisation d'une fermeture dans une structure
struct Cache<T>
// déclaration d'une structure qui dépend de T
where
    T: Fn(u32) -> u32, // T est une fermeture qui prend un entier et retourne un entier
                       // Fn est le nom du trait à implémenter
{
    calcul: T,           // nom la variable qui identifie la fermeture
    valeur: Option<u32>, // résultat retourné par la fermeture (Some ou None)
}

impl<T> Cache<T>
// implémentation du trait Fn dans la structure et du constructeur new
where
    T: Fn(u32) -> u32,
{
    fn new(calcul: T) -> Cache<T> {
        Cache {
            calcul,
            valeur: None,
        }
    }

    fn valeur(&mut self, arg: u32) -> u32 {
        // ajout de la fonction valeur qui renvoi self.valeur si valeur!=None
        match self.valeur {
            Some(v) => v,
            None => {
                let v = (self.calcul)(arg); // et si valeur=None on appel la fermeture self.calcul
                self.valeur = Some(v); // et on stocke le résultat dans self.valeur
                v // et on retourne le résultat
            }
        }
    }
}

fn test_fermeture_structure() {
    // ici on instancie la stucture en aapelant le constructeur en donnant en paramètre la fermeture
    let mut mon_calcul = Cache::new(|nombre| nombre + 1);

    println!("Appel 1 : {}", mon_calcul.valeur(10)); // la fermeture est exécutée
    println!("Appel 2 : {}", mon_calcul.valeur(30)); // le résultat est en cache on renvoi la valeur
    println!("Appel 2 : {}", mon_calcul.valeur(10)); // idem (même si la valeur du paramètre à changé)
}

fn test_iterateur_map() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // on récupère l'itérateur sans l'appeler
    for val in v1_iter {
        println!("On a : {}", val);
    }
    let v1_iter2 = v1.iter();
    let total: i32 = v1_iter2.sum(); // on ne peut pas réutiliser v1_iter car l'itération consomme l'itérateur
    println!("Total : {}", total);
    let min = v1.iter().min(); // idem, il faut redemander un nouvel itérateur
    println!("Min : {:?}", min);

    v1.iter().map(|x| println!("x={}", x + 1)); // ici la fonction anonyme ne sera pas appelée

    let v2: Vec<_> = v1
        .iter()
        .map(|x| {
            println!("dans la fonction anonyme : {}", x);
            x + 1
        })
        .collect(); // le collect va appeler la fonction anonyme pour chaque occurrence
    println!("V2 : {:?}", v2);
}

#[derive(Debug)] // permet d'utiliser debug ou {:?} dans le println! (implémenter automatiquement les traits)
struct Chaussure {
    pointure: u32,
    style: String,
}

fn test_iterateur_filter() {
    let chaussures = vec![
        Chaussure {
            pointure: 10,
            style: String::from("baskets"),
        },
        Chaussure {
            pointure: 13,
            style: String::from("sandale"),
        },
        Chaussure {
            pointure: 10,
            style: String::from("bottes"),
        },
    ];

    let a_ma_pointure: Vec<Chaussure> = chaussures
        .into_iter()
        .filter(|s| s.pointure == 10) // Applique le filtre
        .collect();
    println!("Chaussures à ma pointure : {:?}", a_ma_pointure);

}


struct Compteur {
    compteur: u32,
}

impl Compteur {
    fn new() -> Compteur {
        Compteur { compteur: 0 }
    }
}

impl Iterator for Compteur {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> { // implementation de next qui ici compte j'usqu'à 5 et s'arrête
        if self.compteur < 5 {
            self.compteur += 1;
            Some(self.compteur)
        } else {
            None
        }
    }
}

use std::iter::zip;

fn test_iterateur_custom(){
    let mut compteur = Compteur::new();
    println!("{:?}",compteur.next()); // on a déjà compté jusque 1
    compteur.for_each(|x| {println!("x={}",x);}); // donc là on reprend de 2 à 5

    let test_zip = zip(Compteur::new(),Compteur::new());

    test_zip.for_each(|(x,y)| {println!("test zip x={}, y={}",x,y);});

    let somme: u32 = Compteur::new() // on instancie un premier iterateur compteur
            .zip(Compteur::new().skip(1))// on instancie le second (en zappant le premier élément) et on en crée un 3 ème retourné par zip qui itère sur des tuples : (valeur du premeier compteur, valeur du second) et termine son itération si une des valeurs du tuple est None
            .map(|(a, b)| a * b) // pour chaque occurrence du tuple on multiplie les valeurs du tuple entre elles et on retourne le résultat de la multiplication
            .filter(|x| x % 3 == 0)//et on ne garde que les valeurs divisibles par 3 
            .sum(); // et on somme le résultat du filtre
    println!("{}",somme);
}

fn test_thread(){
    let manipulateur = thread::spawn(|| { // contenu du thread
        for i in 1..10 {
            println!("Bonjour n°{} à partir de la nouvelle tâche !", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Bonjour n°{} à partir de la tâche principale !", i);
        thread::sleep(Duration::from_millis(1));
    }
    manipulateur.join().unwrap(); // force a attendre que les tâches soit terminées


    let v = vec![1, 2, 3];

    let manipulateur = thread::spawn(move || { // avec move le thread prend possession de v, si on met pas move le compilateur ne laisse pas passer car on pourrait dans la tache principale supprimer v alors que le threau l'utilise encore
        println!("Voici un vecteur : {:?}", v);
    });

    manipulateur.join().unwrap();
}

use std::sync::mpsc;

fn test_thread_sender_reciever(){
    let (tx, rx) = mpsc::channel(); // on crée un émetteur et un recever

    thread::spawn(move || { // avec move le thread prend possession du sender
        let valeur = String::from("salut");
        let valeur = String::from("salut");
        println!("La tâche envoie : {valeur}");
        thread::sleep(Duration::from_secs(2));
        tx.send(valeur).unwrap(); // on envoi la valeur mais c'est le récepteur qui en prend possession on ne peut plus utiliser valeur après
        thread::sleep(Duration::from_secs(2));
        tx.send(valeur).unwrap();
    });

    let recu = rx.recv().unwrap(); // le programme se bloque en attendant la réponse
    println!("Le programme principal a reçu: {}", recu);
    let mess = rx.try_recv().unwrap(); // le programme ne se bloque pas en attendant la réponse
    
    println!("{}",mess);
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
    test_table_de_hachage();
    test_panic();
    //test_result();
    //test_result_propagation();
    test_result_propagation_avec_point_interrogation();
    test_structure_generique();
    test_fonction_generique_structure();
    test_fonction_generique_structure2();
    test_trait();
    test_argument_ligne_de_commande();
    test_lecture_fichier_texte();
    test_retour_multiple();
    test_erreur();
    test_sleep();
    test_fermeture_lambda();
    test_fermeture_structure();
    test_iterateur_map();
    test_iterateur_filter();
    test_iterateur_custom();
    test_thread();
    test_thread_sender_reciever();
}
