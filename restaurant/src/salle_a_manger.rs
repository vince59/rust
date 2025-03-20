pub mod salle_a_manger {
    pub mod accueil { // si on ne met pas pub le module reste privé 
        pub fn ajouter_a_la_liste_attente() {} // il faut mettre aussi pub pour utiliser la fonction en dehors du module

        fn installer_a_une_table() {}

        pub mod test1 {
            pub mod test2 {
                fn servir_commande() {}
                pub mod test3 {
                    pub fn test4(){
                        super::servir_commande(); // super fait référence au parent
                    }
                }
            }
        }
    }

    mod service {
        fn prendre_commande() {}

        fn servir_commande() {}

        fn encaisser() {}
    }
}