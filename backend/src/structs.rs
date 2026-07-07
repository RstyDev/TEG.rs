use rand::seq::IteratorRandom;
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, OnceLock},
};
use structs::{Continent, MAP, Tokens};

use uuid::Uuid;
pub fn conquer_two_continents(
    player_id: Uuid,
    status: &HashMap<Uuid, Tokens>,
    c1: Continent,
    c2: Continent,
) -> bool {
    status.iter().all(|st| {
        let country = &MAP.get_or_init(Default::default)[st.0];
        (country.continent() != c1 && country.continent() != c2) || st.1.owner == player_id
    })
}

pub fn conquer_two_plus_some(
    player_id: Uuid,
    status: &HashMap<Uuid, Tokens>,
    c1: Continent,
    c2: Continent,
    others: u8,
) -> bool {
    for (id, country) in status {
        let mut owned = 0u8;
        if MAP.get_or_init(Default::default)[&id].continent() == c1 || MAP.get_or_init(Default::default)[&id].continent() == c2 {
            if country.owner != player_id {
                return false;
            }
        } else {
            if country.owner == player_id {
                owned += 1;
                if owned >= others {
                    return true;
                }
            }
        }
    }
    false
}

pub static COMMON_MISSION: OnceLock<Mission> = OnceLock::new();

pub static MISSIONS: OnceLock<HashMap<&'static str, Mission>> = OnceLock::new();

#[derive(Serialize, Clone)]
pub struct Mission<'a> {
    pub name: &'a str,
    pub objective: Option<Uuid>,
    #[serde(skip)]
    pub exe: Arc<dyn Fn(Uuid, &HashMap<Uuid, Tokens>, Option<Uuid>) -> bool + Send + Sync>,
}
impl Default for Mission<'_> {
    fn default() -> Self {
        Mission {
            name: "Conquer Two Continents",
            objective: None,
            exe: Arc::new(|player_id, status, _| {
                conquer_two_continents(player_id, status, Continent::Europe, Continent::Asia)
            }),
        }
    }
}
impl<'a> Mission<'a> {
    pub fn new_random(player_id: Uuid, players: &Vec<Uuid>) -> Self {
        //todo!("Agregar current missions para no repetirlas");
        let mission_name = *MISSIONS.get().unwrap().keys().choose(&mut rand::rng()).unwrap();
        let mission = MISSIONS.get().unwrap().get(mission_name).unwrap();
        if mission_name.contains("Eliminar") {
            let target_player_id = players
                .into_iter()
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
    pub fn name(&self) -> String {
        match self.objective {
            Some(id) => format!("{}{}", self.name, id),
            None => {
                if self.name.contains("Eliminar") {
                    COMMON_MISSION.get().unwrap().name()
                } else {
                    self.name.to_string()
                }
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

pub fn initialize() {
    MISSIONS
        .set({
            let mut map = HashMap::new();
            let m1 = "Conquistar Asia y América del Sur";
            let _f1 = Mission {
                name: m1,
                exe: Arc::new(|player_id, status, _| {
                    conquer_two_continents(
                        player_id,
                        status,
                        Continent::Asia,
                        Continent::SouthAmerica,
                    )
                }),
                objective: None,
            };
            map.insert(m1, _f1);
            let m2 = "Conquistar Asia y África";
            let _f2 = Mission {
                name: m2,
                exe: Arc::new(|player_id, status, _| {
                    conquer_two_continents(player_id, status, Continent::Asia, Continent::Africa)
                }),
                objective: None,
            };
            map.insert(m2, _f2);
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
                        5,
                    )
                }),
                objective: None,
            };
            map.insert(m4, _f4);
            let m5 = "Conquistar Europa, América del Sur y 6 países adicionales";
            let _f5 = Mission {
                name: m5,
                exe: Arc::new(|player_id, status, _| {
                    conquer_two_plus_some(
                        player_id,
                        status,
                        Continent::Europe,
                        Continent::SouthAmerica,
                        6,
                    )
                }),
                objective: None,
            };
            map.insert(m5, _f5);
            // let m6 = "Conquistar Europa, Oceanía y 6 países adicionales";
            // let _f6 = Mission {
            //     name: m6,
            //     exe: Arc::new(|player_id, status, _| {
            //         conquer_two_plus_some(player_id, status, Continent::Europe, Continent::Oceania, 6)
            //     }),
            //     objective: None,
            // };
            let m6 = "Conquistar Europa, África y 6 países adicionales";
            let _f6 = Mission {
                name: m6,
                exe: Arc::new(|player_id, status, _| {
                    conquer_two_plus_some(
                        player_id,
                        status,
                        Continent::Europe,
                        Continent::Africa,
                        6,
                    )
                }),
                objective: None,
            };
            map.insert(m6, _f6);
            let m7 = "Conquistar Asia y Europa";
            let _f7 = Mission {
                name: m7,
                exe: Arc::new(|player_id, status, _| {
                    conquer_two_continents(player_id, status, Continent::Asia, Continent::Europe)
                }),
                objective: None,
            };
            map.insert(m7, _f7);
            let m8 = "Conquistar Oceanía, América del Sur y 9 países adicionales";
            let _f8 = Mission {
                name: m8,
                exe: Arc::new(|player_id, status, _| {
                    conquer_two_plus_some(
                        player_id,
                        status,
                        Continent::Oceania,
                        Continent::SouthAmerica,
                        9,
                    )
                }),
                objective: None,
            };
            map.insert(m8, _f8);
            let m9 = "Conquistar 3 continentes completos";
            let _f9 = Mission {
                name: m9,
                exe: Arc::new(|player_id, status, _| {
                    let mut continents_owned = 0u8;
                    for continent in Continent::default().into_iter() {
                        if status
                            .into_iter()
                            .filter(|(id, c)| MAP.get().unwrap()[id].continent() == continent)
                            .all(|(_, c)| c.owner == player_id)
                        {
                            continents_owned += 1;
                        }
                    }
                    continents_owned >= 3
                }),
                objective: None,
            };
            map.insert(m9, _f9);
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
            map.insert(m10, _f10);
            let m11 = "Conquistar Europa, Oceanía y 8 países adicionales"; //21
            let _f11 = Mission {
                name: m11,
                objective: None,
                exe: Arc::new(|player_id, status, _| {
                    conquer_two_plus_some(
                        player_id,
                        status,
                        Continent::Europe,
                        Continent::Oceania,
                        8,
                    )
                }),
            };
            map.insert(m11, _f11);
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
            map.insert(m12, _f12);
            let m13 = "Conquistar Asia, Oceanía y 2 países adicionales";
            let _f13 = Mission {
                name: m13,
                objective: None,
                exe: Arc::new(|player_id, status, _| {
                    conquer_two_plus_some(player_id, status, Continent::Asia, Continent::Oceania, 2)
                }),
            };
            map.insert(m13, _f13);
            let m14 = "Conquistar 24 países con dos o más tropas en cada uno";
            let _f14 = Mission {
                name: m14,
                objective: None,
                exe: Arc::new(|player_id, status, _| {
                    status
                        .into_iter()
                        .filter(|(id, st)| st.owner == player_id && st.amount >= 2)
                        .count()
                        >= 24
                }),
            };
            map.insert(m14, _f14);
            let m15 = "Eliminar al jugador: ";
            let _f15 = Mission {
                name: m15,
                objective: None,
                exe: Arc::new(|player_id, status, objective| match objective {
                    Some(obj) => status.into_iter().all(|(_, st)| st.owner != obj),
                    None => (&*COMMON_MISSION.get_or_init(||Default::default()).exe)(player_id, status, None),
                }),
            };
            map.insert(m15, _f15);
            map
        })
        .unwrap();
    COMMON_MISSION
        .set(Mission {
            name: "Conquista 30 países",
            objective: None,
            exe: Arc::new(|player_id, status, _| {
                status
                    .into_iter()
                    .filter(|(_, st)| st.owner == player_id)
                    .count()
                    >= 30
            }),
        })
        .unwrap();
}
