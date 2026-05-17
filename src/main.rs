use clap::{Parser, builder::Str};
use random_word::Lang;
use unaccent::unaccent;

#[derive(Parser)]
struct Args {
    // Le nom de la table dans laquelle les données vont être ajoutées
    table: String,

    // La liste des colonnes de la table
    colonnes: String,

    // La liste des types des colonnes de la table
    types: String,

    // Le nombre de lignes à générer
    nombre_de_lignes: u16
}

fn main() {
    // On récupère les arguments
    let args = Args::parse();

    // On décompose colonnes et types en vecteurs
    let colonnes = string_to_vec(&args.colonnes);
    let types = string_to_vec(&args.types);

    // On vérifie si il y a autant de colonnes que de types
    assert_eq!(colonnes.len(), types.len(), "Il doit y avoir autant de colonnes que de types !");

    let mut res: String = String::new();

    // On génère autant de lignes que voulu
    for _ in 0..args.nombre_de_lignes {
        let mut ligne: String = String::from(format!("insert into {} ({}) values (", args.table, args.colonnes));

        // On génère autant de valeurs que de colonnes
        for i in 0..colonnes.len() {
            let valeur: String = generer_valeur(&types[i], &colonnes[i]);
            if i < colonnes.len() - 1 {
                // On ajoute la valeur et une virgule
                ligne.push_str(&valeur);
                ligne.push_str(&",");
            } else {
                // On ajoute la valeur sans virgule parce que c'est la dernière
                ligne.push_str(&valeur);
            }
        }

        ligne.push_str(&");\n");

        res.push_str(&ligne);
    }

    print!("{}", res);
}

// Transforme une chaîne de caractères en vecteur, en considérant que les éléments sont séparés par des virgules
fn string_to_vec(ch: &String) -> Vec<String> {
    // Les variables
    let mut mot_en_cours: String = String::from("");
    let mut vec_res: Vec<String> = Vec::new();
    // On enlève les accents pour éviter les erreurs dûes à la taille des caractères
    let chaine = unaccent(ch);

    for i in 0..chaine.len() {
        if chaine.chars().nth(i) != Some(',') {
            mot_en_cours.push_str(&String::from(chaine.chars().nth(i).unwrap()));     // On récupère le caractère dans l'option avec unwrap, et on le convertit en Str
        } else {
            vec_res.push(String::from(&mot_en_cours));
            mot_en_cours = String::from("");
        }
    }
    // On pousse le dernier mot
    vec_res.push(String::from(&mot_en_cours));

    vec_res
}

// Génère une valeur en fonction du type donné et potentiellement du nom de la table
fn generer_valeur(type_valeur: &String, colonne: &String) -> String {
    // La variable résultat
    let mut res: String = String::new();

    // Si la valeur est de type texte
    if vec![&String::from("TEXT"), &String::from("VARCHAR")].contains(&&type_valeur.to_uppercase()) {
        // On ajoute des guillemets autour des textes
        res.push_str(&String::from("'"));
        
        // Si la valeur est un nom
        if vec![&String::from("NOM"), &String::from("NAME"), &String::from("LASTNAME"), &String::from("LAST NAME"), &String::from("LAST-NAME"), &String::from("LAST_NAME")].contains(&&colonne.to_uppercase()) {
            // Pour l'instant on fait un vecteur avec toutes les possibilités, il faudra voir plus tard si c'est possible de les stocker dans un fichier
            // Liste issue des noms les plus courants par pays, selon le site https://patronymes.com/pays-liste le 16 mai 2026 + quelques ajouts
            let noms: Vec<String> = vec![String::from("Habibullah"), String::from("Dlamini"), String::from("Hoxha"), String::from("Saidi"), String::from("Müller"), String::from("Garcia"), String::from("Smith"), String::from("Manuel"), String::from("Richardson"), String::from("Joseph"), String::from("Khan"), String::from("Gonzalez"), String::from("Grigoryan"), String::from("Tromp"), String::from("Gruber"), String::from("Mammadova"), String::from("Rolle"), String::from("Ali"), String::from("Akter"), String::from("Clarke"), String::from("Peeters"), String::from("Martinez"), String::from("Dorji"), String::from("Mamani"), String::from("Hodžić"), String::from("Molefe"), String::from("Haji"), String::from("Da Silva"), String::from("Ivanov"), String::from("Ouedraogo"), String::from("Nkurunziza"), String::from("Bio"), String::from("Sok"), String::from("Ngo"), String::from("Lopes"), String::from("Wang"), String::from("Georgiou"), String::from("Mohamed"), String::from("Kim"), String::from("Knežević"), String::from("Kone"), String::from("Jensen"), String::from("Hernandez"), String::from("Tamm"), String::from("Kumar"), String::from("Korhonen"), String::from("Martin"), String::from("Ndong"), String::from("Jallow"), String::from("Mensah"), String::from("Charles"), String::from("Olsen"), String::from("Papadopoulos"), String::from("Cruz"), String::from("Lopez"), String::from("Le page"), String::from("Diallo"), String::from("Nguema"), String::from("Gomes"), String::from("Persaud"), String::from("Beridze"), String::from("Jean"), String::from("Chan"), String::from("Tóth"), String::from("Kelly"), String::from("Devi"), String::from("Sari"), String::from("Mohammadi"), String::from("Murphy"), String::from("Campbell"), String::from("Jónsdóttir"), String::from("Cohen"), String::from("Rossi"), String::from("Brown"), String::from("Tanaka"), String::from("Allah"), String::from("Mwangi"), String::from("Ismailova"), String::from("Ioane"), String::from("Krasniqi"), String::from("Ali"), String::from("Inthavong"), String::from("Mohapi"), String::from("Berzina"), String::from("El Din"), String::from("Kollie"), String::from("Büchel"), String::from("Petrauskas"), String::from("Schmit"), String::from("Stojanovski"), String::from("Rakotomalala"), String::from("Tan"), String::from("Banda"), String::from("Traore"), String::from("Borg"), String::from("Alaoui"), String::from("Beeharry"), String::from("Ba"), String::from("Mori"), String::from("Rusu"), String::from("Ganbold"), String::from("Weekes"), String::from("Popović"), String::from("Langa"), String::from("Maung"), String::from("Johannes"), String::from("Harris"), String::from("Abdou"), String::from("Ibrahim"), String::from("Talagi"), String::from("Christian"), String::from("Hansen"), String::from("Dubois"), String::from("Caudhari"), String::from("Gawas"), String::from("Akello"), String::from("Tellei"), String::from("John"), String::from("Jones"), String::from("De Jong"), String::from("Dela Cruz"), String::from("Nowak"), String::from("Wong"), String::from("Silva"), String::from("Sanchez"), String::from("Quispe"), String::from("Popa"), String::from("Ivanova"), String::from("Uwimana"), String::from("Nováková"), String::from("Moussa"), String::from("Ngoma"), String::from("Ilunga"), String::from("Questel"), String::from("Lake"), String::from("Detcheverry"), String::from("Williams"), String::from("Gasperoni"), String::from("Yon"), String::from("Meredith"), String::from("Fuimaono"), String::from("Fernandes"), String::from("Jovanović"), String::from("Hoareau"), String::from("Kamara"), String::from("Varga"), String::from("Novak"), String::from("Ahmed"), String::from("Perera"), String::from("Deng"), String::from("Lin"), String::from("Johansson"), String::from("Khaled"), String::from("Ndiaye"), String::from("Sharipova"), String::from("Chen"), String::from("Juma"), String::from("Mahamat"), String::from("Awad"), String::from("Saetang"), String::from("Soares"), String::from("Lawson"), String::from("Latu"), String::from("Charyeva"), String::from("Yilmaz"), String::from("Melnik"), String::from("Tari"), String::from("Bachmann"), String::from("Nguyen"), String::from("Vegi"), String::from("Phiri"), String::from("Moyo"), String::from("Tesfaye"), String::from("Ebanks"), String::from("Marsters"), String::from("Betts"), String::from("Joensen"), String::from("Sablan"), String::from("Mae"), String::from("Holmes"), String::from("Lupin"), String::from("Torvald"), String::from("Gil Sayan"), String::from("Duciel"), String::from("Watson")];

            let rand: usize = fastrand::usize(0..noms.len() - 1);

            res.push_str(&noms[rand]);
        } else if vec![&String::from("PRENOM"), &String::from("FIRSTNAME"), &String::from("FIRST-NAME"), &String::from("FIRST_NAME"), &String::from("FIRST NAME")].contains(&&colonne.to_uppercase()) {
            // Prénoms issues de cette page wikipédia : https://en.wikipedia.org/wiki/Alice_and_Bob + quelques ajouts
            let prenoms: Vec<String> = vec![String::from("Alice"), String::from("Bob"), String::from("Charlie"), String::from("Craig"), String::from("Dave"), String::from("Eve"), String::from("Frank"), String::from("Grace"), String::from("Heidi"), String::from("Ivan"), String::from("Judy"), String::from("Mallory"), String::from("Michael"), String::from("Niaj"), String::from("Olivia"), String::from("Oscar"), String::from("Pat"), String::from("Rupert"), String::from("Sybil"), String::from("Ted"), String::from("Trudy"), String::from("Vanna"), String::from("Walter"), String::from("Wendy"), String::from("Trucmuche"), String::from("Elvendork"), String::from("Ewilan"), String::from("Camille"), String::from("Sherlock"), String::from("John"), String::from("Marc"), String::from("Salim"), String::from("Achille"), String::from("Patrocle"), String::from("Ulysse"), String::from("Hector"), String::from("Paris"), String::from("Briseis"), String::from("Penelope"), String::from("Toto")];
        
            let rand: usize = fastrand::usize(0..prenoms.len() - 1);

            res.push_str(&prenoms[rand]);
        } else if vec![&String::from("MOT DE PASSE"), &String::from("MOTDEPASSE"), &String::from("MDP"), &String::from("PASSWORD"), &String::from("PASSWD"), &String::from("MOT_DE_PASSE"), &String::from("MOT-DE-PASSE")].contains(&&colonne.to_uppercase()){
            // Ne donnez pas ces mots de passes à des administrateurs !
            let mdps: Vec<String> = vec![String::from("JaimeLesPates123"), String::from("MonChienCestLeMeilleur"), String::from("LesChaussettesDeLArchiduchesseSontEllesSechesArchiSeches"), String::from("LesHackersVousDevinerezJamais"), String::from("MonSuperMotDePasseInviolable"), String::from("VousSavezJeNeCroisPasQuIlYAitDeBonnesOuDeMauvaisesSituations"), String::from("LaCigaleAyantChantéToutLÉtéSeTrouvaFortDépourvueQuandLaBriseFutVenue"), String::from("CesameOuvreToiMaisPasTropViteQuandMeme"), String::from("LeWAQTEstUnAnimalAquatiqueNocturne"), String::from("TuSaisPourquoi"), String::from("ILoveRust"), String::from("SiTonTontonTondTonTontonTonTontonSeraTondu"), String::from("ÊtreOuNÊtrePasTelleEstLaQuestion"), String::from("MyNameIsNobody"), String::from("WouldYouFallInLoveWithMeAgain?IfYouKnewAllIveDoneTheThingsIcannotChangeWouldYouLoveMeAllTheSame?IKnowThatYouveBeenWaitingWaiiitingForLove")];
        
            let rand: usize = fastrand::usize(0..mdps.len() - 1);

            res.push_str(&mdps[rand]);

        } else {
            // On choisit un mot aléatoire grâce à la crate random_word
            res.push_str(&random_word::get(Lang::Fr).to_string());
        }

        res.push_str(&String::from("'"));

    } else if vec![&String::from("INTEGER"), &String::from("INT")].contains(&&type_valeur.to_uppercase()) {

        if &"AGE" == &colonne.to_uppercase() {
            // On génère un âge entre 7 et 77 ans
            res.push_str(&fastrand::i8(7..77).to_string());

        } else if vec![&String::from("YEAR"), &String::from("ANNEE"), &String::from("AN")].contains(&&colonne.to_uppercase()) {
            res.push_str(&fastrand::i16(1881..2222).to_string());
        }

        // On choisit un entier aléatoire
        res.push_str(&fastrand::u32(..).to_string());

    } else if vec![&String::from("REAL"), &String::from("FLOAT"), &String::from("DOUBLE")].contains(&&type_valeur.to_uppercase()) {
        // On chosit un flottant aléatoire
        res.push_str(&fastrand::f32().to_string());

    } else if &"NULL" == &&type_valeur.to_uppercase() {
        // La valeur NULL
        res.push_str(&"NULL");

    } else if &"BOOLEAN" == &&type_valeur.to_uppercase() {
        // Une valeur aléatoire entre 0 et 1
        res.push_str(&fastrand::u8(0..1).to_string());
    } else {
        // Gestion du BLOB et du reste, on renvoit un entier
        res.push_str(&fastrand::u32(..).to_string());
    }

    res
}
