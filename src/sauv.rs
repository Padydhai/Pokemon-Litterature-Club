//Pour avoir une structure de sauvegarde
struct Indication {
    sauvegarde: bool, //Si une sauvegarde existe ou non
    dialogue: usize,  //Dernier dialogue parcouru
}

impl GameSettings{
    pub const fn new()->Indication{ //Pour une nouvelle partie, ...
        Self{
            sauvegarde:false, //Il n'y a pas de sauvegarde.
            dialogue: 0, //Le jeu est à l'état "0".
        }
    }

