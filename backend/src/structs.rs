use macros::hashmap;
use rand::seq::IteratorRandom;
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, LazyLock},
};
use structs::{CStatus, Continent, MAP};

use uuid::Uuid;
fn conquer_two_continents(
    player_id: Uuid,
    status: &HashMap<Uuid, CStatus>,
    c1: Continent,
    c2: Continent,
) -> bool {
    status.iter().all(|st| {
        let country = &MAP[st.0];
        (country.continent() != c1 && country.continent() != c2)
            || st.1.tokens.as_ref().map_or(false, |t| t.owner == player_id)
    })
}

fn conquer_two_plus_some(
    player_id: Uuid,
    status: &HashMap<Uuid, CStatus>,
    c1: Continent,
    c2: Continent,
    others: u8,
) -> bool {
    for (id, country) in status {
        let mut owned = 0u8;
        if MAP[&id].continent() == c1 || MAP[&id].continent() == c2 {
            if country
                .tokens
                .as_ref()
                .map_or(true, |tk| tk.owner != player_id)
            {
                return false;
            }
        } else {
            if country
                .tokens
                .as_ref()
                .map_or(false, |tk| tk.owner == player_id)
            {
                owned += 1;
                if owned >= others {
                    return true;
                }
            }
        }
    }
    false
}

pub static COMMON_MISSION: LazyLock<Mission> = LazyLock::new(|| Mission {
    name: "Conquista 30 países",
    objective: None,
    exe: Arc::new(|player_id, status, _| {
        status
            .into_iter()
            .filter(|(_, st)| st.tokens.as_ref().map_or(false, |tk| tk.owner == player_id))
            .count()
            >= 30
    }),
});

pub static MISSIONS: LazyLock<HashMap<&'static str, Mission>> = LazyLock::new(|| {
    let m1 = "Conquistar Asia y América del Sur";
    let _f1 = Mission {
        name: m1,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_continents(player_id, status, Continent::Asia, Continent::SouthAmerica)
        }),
        objective: None,
    };
    let m2 = "Conquistar Asia y África";
    let _f2 = Mission {
        name: m2,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_continents(player_id, status, Continent::Asia, Continent::Africa)
        }),
        objective: None,
    };
    // let m3 = "Conquistar América del Norte, África y 5 países adicionales";
    // let _f3 = Mission {
    //     name: m3,
    //     exe: Arc::new(|player_id, status, _| {
    //         conquer_two_plus_some(
    //             player_id,
    //             status,
    //             Continent::NorthAmerica,
    //             Continent::Africa,
    //             5
    //         )
    //     }),
    //     objective: None,
    // };
    let m4 = "Conquistar América del Norte, Oceanía y 5 países adicionales";
    let _f4 = Mission {
        name: m4,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_plus_some(
                player_id,
                status,
                Continent::NorthAmerica,
                Continent::Oceania,
                5
            )
        }),
        objective: None,
    };
    let m5 = "Conquistar Europa, América del Sur y 6 países adicionales";
    let _f5 = Mission {
        name: m5,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_plus_some(
                player_id,
                status,
                Continent::Europe,
                Continent::SouthAmerica,
                6
            )
        }),
        objective: None,
    };
    // let m6 = "Conquistar Europa, Oceanía y 6 países adicionales";
    // let _f6 = Mission {
    //     name: m6,
    //     exe: Arc::new(|player_id, status, _| {
    //         conquer_two_plus_some(player_id, status, Continent::Europe, Continent::Oceania, 6)
    //     }),
    //     objective: None,
    // };
    let m7 = "Conquistar Europa, África y 6 países adicionales";
    let _f7 = Mission {
        name: m7,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_plus_some(player_id, status, Continent::Europe, Continent::Africa, 6)
        }),
        objective: None,
    };
    let m8 = "Conquistar Asia y Europa";
    let _f8 = Mission {
        name: m8,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_continents(player_id, status, Continent::Asia, Continent::Europe)
        }),
        objective: None,
    };
    let m9 = "Conquistar Oceanía, América del Sur y 9 países adicionales";
    let _f9 = Mission {
        name: m9,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_plus_some(
                player_id,
                status,
                Continent::Oceania,
                Continent::SouthAmerica,
                9
            )
        }),
        objective: None,
    };
    let m10 = "Conquistar 3 continentes completos";
    let _f10 = Mission {
        name: m10,
        exe: Arc::new(|player_id, status, _| {
            let mut continents_owned = 0u8;
            for continent in Continent::default().into_iter() {
                if status
                    .iter()
                    .filter(|(_, c)| MAP[&c.country_id].continent() == continent)
                    .all(|(_, c)| c.tokens.as_ref().map_or(false, |t| t.owner == player_id))
                {
                    continents_owned += 1;
                }
            }
            continents_owned >= 3
        }),
        objective: None,
    };
    let m10 = "Conquistar América del Sur, África y 5 países adicionales";
    let _f10 = Mission {
        name: m10,
        objective: None,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_plus_some(
                player_id,
                status,
                Continent::SouthAmerica,
                Continent::Africa,
                5,
            )
        }),
    };
    let m11 = "Conquistar Europa, Oceanía y 8 países adicionales"; //21
    let _f11 = Mission {
        name: m11,
        objective: None,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_plus_some(player_id, status, Continent::Europe, Continent::Oceania, 8)
        }),
    };
    let m12 = "Conquistar América del Norte, África y 4 países adicionales";
    let _f12 = Mission {
        name: m12,
        objective: None,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_plus_some(
                player_id,
                status,
                Continent::NorthAmerica,
                Continent::Africa,
                4,
            )
        }),
    };
    let m13 = "Conquistar Asia, Oceanía y 2 países adicionales";
    let _f13 = Mission {
        name: m13,
        objective: None,
        exe: Arc::new(|player_id, status, _| {
            conquer_two_plus_some(player_id, status, Continent::Asia, Continent::Oceania, 2)
        }),
    };
    let m14 = "Conquistar 24 países con dos o más tropas en cada uno";
    let _f14 = Mission {
        name: m14,
        objective: None,
        exe: Arc::new(|player_id, status, _| {
            status
                .into_iter()
                .filter(|(id, st)| {
                    st.tokens
                        .as_ref()
                        .map_or(false, |tk| tk.owner == player_id && tk.amount >= 2)
                })
                .count()
                >= 24
        }),
    };
    let m15 = "Eliminar al jugador: ";
    let _f15 = Mission {
        name: m15,
        objective: None,
        exe: Arc::new(|player_id, status, objective| match objective {
            Some(obj) => status
                .into_iter()
                .all(|(_, st)| st.tokens.as_ref().map_or(false, |tk| tk.owner != obj)),
            None => (COMMON_MISSION.exe)(player_id, status, None),
        }),
    };
    hashmap!(
        m1:_f1, m2:_f2, //m3:_f3, 
        m4:_f4, m5:_f5, //m6:_f6, 
        m7:_f7, m8:_f8, m9:_f9, 
        m10:_f10, m11:_f11, m12:_f12, 
        m13:_f13, m14:_f14, m15:_f15
    )
});

#[derive(Serialize, Clone)]
pub struct Mission<'a> {
    name: &'a str,
    objective: Option<Uuid>,
    #[serde(skip)]
    exe: Arc<dyn Fn(Uuid, &HashMap<Uuid, CStatus>, Option<Uuid>) -> bool + Send + Sync>,
}

impl<'a> Mission<'a> {
    pub fn new_random(player_id: Uuid, players: &HashMap<Uuid, CStatus>) -> Self {
        let mission_name = MISSIONS.keys().choose(&mut rand::rng()).unwrap();
        let mission = MISSIONS.get(mission_name).unwrap();
        if mission_name.contains("Eliminar") {
            let target_player_id = players
                .keys()
                .filter(|id| **id != player_id)
                .choose(&mut rand::rng())
                .unwrap();
            Mission {
                name: mission.name,
                objective: Some(*target_player_id),
                exe: mission.exe.clone(),
            }
        } else {
            Mission {
                name: mission.name,
                objective: None,
                exe: mission.exe.clone(),
            }
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
