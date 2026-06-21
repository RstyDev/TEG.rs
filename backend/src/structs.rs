use std::{collections::HashMap, fmt::Debug, sync::{Arc, LazyLock}};
use macros::hashmap;
use rand::seq::IteratorRandom;
use serde::Serialize;
use structs::{CStatus, Continent, MAP};

use uuid::Uuid;
pub static MISSIONS: LazyLock<HashMap<&'static str, Mission>> = LazyLock::new(||{
    let m1 = "Conquistar Asia y América del Sur";
    let _f1 = Mission { name: m1, exe: Arc::new(|player_id, status,_|{
        status.iter().all(|st|{
            let country = MAP.get(st.0).unwrap();
            country.continent() != Continent::Asia && country.continent() != Continent::SouthAmerica || st.1.tokens.as_ref().map_or(false, |t| t.owner == player_id)
        })
    }),
        objective: None, };
    let m2 = "Conquistar Asia y África";
    let _f2 = Mission { name: m2, exe: Arc::new(|player_id,status,_|{
        status.iter().all(|st|{
            let country = MAP.get(st.0).unwrap();
            country.continent() != Continent::Asia && country.continent() != Continent::Africa || st.1.tokens.as_ref().map_or(false, |t| t.owner == player_id)
        })
    }),
        objective: None, };
    let m3 = "Conquistar América del Norte y África";
    let _f3 = Mission { name: m3, exe: Arc::new(|player_id,status,_|{
        status.iter().all(|st|{
            let country = MAP.get(st.0).unwrap();
            country.continent() != Continent::NorthAmerica && country.continent() != Continent::Africa || st.1.tokens.as_ref().map_or(false, |t| t.owner == player_id)
        })
    }),
        objective: None, };
    /*
Conquistar América del Norte y Oceanía
Conquistar Europa y América del Sur
Conquistar Europa y Oceanía
Conquistar Europa y África
Conquistar Asia y Europa
Conquistar Oceanía y América del Sur */
    hashmap!(m1:_f1, m2:_f2, m3:_f3)
});


#[derive(Serialize, Clone)]
pub struct Mission<'a> {
    name: &'a str,
    objective: Option<Uuid>,
    #[serde(skip)]
    exe: Arc<dyn Fn(Uuid,HashMap<Uuid, CStatus>,Option<Uuid>) -> bool + Send + Sync>,
}

impl<'a> Mission<'a> {
    pub fn new_random(player_id: Uuid, players: &HashMap<Uuid, CStatus>) -> Self {
        let mission_name = MISSIONS.keys().choose(&mut rand::rng()).unwrap();
        let mission = MISSIONS.get(mission_name).unwrap();
        if mission_name.contains("Eliminar") {
            let target_player_id = players.keys().filter(|id| **id != player_id).choose(&mut rand::rng()).unwrap();
            Mission { name: mission.name, objective: Some(*target_player_id), exe: mission.exe.clone() }
        } else {
            Mission { name: mission.name, objective: None, exe: mission.exe.clone() }
        }
    }
}

impl Debug for Mission<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mission")
            .field("name", &self.name)
            .field("objective", &self.objective)
            .finish()
    }
}