//cargo new --lib restaurant

mod salle_a_manger; // indique que le module existe mais dans un autre fichier qui porte le même nom que le module

fn servir_commande() {} // on peut avoir plusieurs fois la même fonction tant qu'elle est dans des modules différents

mod cuisines {
    fn corriger_commande_erronee() {
        cuisiner_commande();
        super::servir_commande(); // c'est un chemin relatif comme ../ on revient un cran en ariere
    }

    pub struct PetitDejeuner { // on peut déclarer la structure publique mais les champs sont privés par défaut
        pub tartine_grillee: String, // sauf si on les déclare public
        fruit_de_saison: String,
    }

    impl PetitDejeuner {
        pub fn en_ete(tartine_grillee: &str) -> PetitDejeuner { // constructeur qui retourne une instance de la structure
            PetitDejeuner {
                tartine_grillee: String::from(tartine_grillee),
                fruit_de_saison: String::from("pêches"),
            }
        }
    }
    fn cuisiner_commande() {}
    pub enum AmuseBouche { // pour un enum si il est public toutes ses variantes le sont aussi
        Soupe,
        Salade,
    }
}

pub fn manger_au_restaurant() {
    // Chemin absolu on commence avec le mot clé crate
    //crate::salle_a_manger::accueil::ajouter_a_la_liste_attente();

    // Chemin relatif
    accueil::ajouter_a_la_liste_attente();
    let commande1 = cuisines::AmuseBouche::Soupe;
    let commande2 = cuisines::AmuseBouche::Salade;
}

pub fn manger_au_restaurant2() {
    // On commande un petit-déjeuner en été avec tartine grillée au seigle
    let mut repas = cuisines::PetitDejeuner::en_ete("seigle");
    // On change d'avis sur le pain que nous souhaitons
    repas.tartine_grillee = String::from("blé");
    println!( "Je voudrais une tartine grillée au {}, s'il vous plaît.",
              repas.tartine_grillee);

    // repas.fruit_de_saison = String::from("myrtilles"); // plante car fruit_de_saison est privé
}

//use crate::salle_a_manger::accueil; // permet de rendre accueil accessible sans mettre les prefixes

//pub use crate::salle_a_manger::accueil; // importe le module accueil et le reéxporte ce qui permet aucode externe de l'utiliser

pub fn manger_au_restaurant3() {
    accueil::ajouter_a_la_liste_attente();
}
use crate::salle_a_manger::salle_a_manger::accueil;
use std::fmt;
use std::io;

fn fonction1() -> fmt::Result {
    Ok(())
}

fn fonction2() -> io::Result<()> {
    Ok(())
}

//use std::{cmp::Ordering, io}; // évite d'écrire une ligne par import
// est équivallent à :
//use std::cmp::Ordering;
//use std::io;

//use std::io::{self, Write}; // on peut le faire aussi depuis n'importe quel emplacement du module

use std::collections::*; // importe tous les éléments public du niveau
