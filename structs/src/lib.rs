use macros::hashmap;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, fmt::{Debug, Display}, sync::{LazyLock, OnceLock},
};
use uuid::Uuid;
pub static MAP: OnceLock<HashMap<Uuid, Country>> = OnceLock::new();
pub fn initialize_map(){
    MAP.set({
        let mut arg = Country::new(None, Continent::SouthAmerica, CName::Argentina);
        let mut chile = Country::new(None, Continent::SouthAmerica, CName::Chile);
        chile.add_adjacent(arg.id());
        arg.add_adjacent(chile.id());
        let arg_id = arg.id();
        let chile_id = chile.id();
        let mut uru = Country::new(None, Continent::SouthAmerica, CName::Uruguay);
        uru.add_adjacent(arg_id);
        arg.add_adjacent(uru.id());
        let uru_id = uru.id();
        let mut bra = Country::new(None, Continent::SouthAmerica, CName::Brasil);
        let bra_id = bra.id();
        bra.add_adjacent(arg_id);
        arg.add_adjacent(bra_id);
        bra.add_adjacent(uru_id);
        uru.add_adjacent(bra_id);
        let mut peru = Country::new(None, Continent::SouthAmerica, CName::Perú);
        let peru_id = peru.id();
        peru.add_adjacent(arg_id);
        arg.add_adjacent(peru_id);
        chile.add_adjacent(peru_id);
        peru.add_adjacent(chile_id);
        bra.add_adjacent(peru_id);
        peru.add_adjacent(bra_id);
        let mut col = Country::new(None, Continent::SouthAmerica, CName::Colombia);
        let col_id = col.id();
        col.add_adjacent(peru_id);
        peru.add_adjacent(col_id);
        bra.add_adjacent(col_id);
        col.add_adjacent(bra_id);
        let mut mex = Country::new(None, Continent::NorthAmerica, CName::México);
        let mex_id = mex.id();
        mex.add_adjacent(col_id);
        col.add_adjacent(mex_id);
        let mut cal = Country::new(None, Continent::NorthAmerica, CName::California);
        let cal_id = cal.id();
        cal.add_adjacent(mex_id);
        mex.add_adjacent(cal_id);
        let mut ny = Country::new(None, Continent::NorthAmerica, CName::NuevaYork);
        let ny_id = ny.id();
        ny.add_adjacent(cal_id);
        cal.add_adjacent(ny_id);
        let mut ore = Country::new(None, Continent::NorthAmerica, CName::Oregón);
        let ore_id = ore.id();
        ore.add_adjacent(cal_id);
        cal.add_adjacent(ore_id);
        ore.add_adjacent(ny_id);
        ny.add_adjacent(ore_id);
        let mut alaska = Country::new(None, Continent::NorthAmerica, CName::Alaska);
        let alaska_id = alaska.id();
        alaska.add_adjacent(ore_id);
        ore.add_adjacent(alaska_id);
        let mut yukon = Country::new(None, Continent::NorthAmerica, CName::Yukón);
        let yukon_id = yukon.id();
        yukon.add_adjacent(alaska_id);
        alaska.add_adjacent(yukon_id);
        yukon.add_adjacent(ore_id);
        ore.add_adjacent(yukon_id);
        let mut canada = Country::new(None, Continent::NorthAmerica, CName::Canadá);
        let canada_id = canada.id();
        canada.add_adjacent(yukon_id);
        yukon.add_adjacent(canada_id);
        canada.add_adjacent(ore_id);
        ore.add_adjacent(canada_id);
        canada.add_adjacent(ny_id);
        ny.add_adjacent(canada_id);
        let mut terranova = Country::new(None, Continent::NorthAmerica, CName::Terranova);
        let terranova_id = terranova.id();
        terranova.add_adjacent(ny_id);
        ny.add_adjacent(terranova_id);
        terranova.add_adjacent(canada_id);
        canada.add_adjacent(terranova_id);
        let mut labrador = Country::new(None, Continent::NorthAmerica, CName::Labrador);
        let labrador_id = labrador.id();
        labrador.add_adjacent(terranova_id);
        terranova.add_adjacent(labrador_id);
        let mut gro = Country::new(None, Continent::NorthAmerica, CName::Groenlandia);
        let gro_id = gro.id();
        gro.add_adjacent(labrador_id);
        labrador.add_adjacent(gro_id);
        gro.add_adjacent(ny_id);
        ny.add_adjacent(gro_id);
        let mut islandia = Country::new(None, Continent::Europe, CName::Islandia);
        let islandia_id = islandia.id();
        gro.add_adjacent(islandia_id);
        islandia.add_adjacent(gro_id);
        let mut brit = Country::new(None, Continent::Europe, CName::GranBretaña);
        let brit_id = brit.id();
        brit.add_adjacent(islandia_id);
        islandia.add_adjacent(brit_id);
        let mut suecia = Country::new(None, Continent::Europe, CName::Suecia);
        let suecia_id = suecia.id();
        suecia.add_adjacent(islandia_id);
        islandia.add_adjacent(suecia_id);
        let mut rusia = Country::new(None, Continent::Europe, CName::Rusia);
        let rusia_id = rusia.id();
        suecia.add_adjacent(rusia_id);
        rusia.add_adjacent(suecia_id);
        let mut polonia = Country::new(None, Continent::Europe, CName::Polonia);
        let polonia_id = polonia.id();
        rusia.add_adjacent(polonia_id);
        polonia.add_adjacent(rusia_id);
        let mut alemania = Country::new(None, Continent::Europe, CName::Alemania);
        let alemania_id = alemania.id();
        polonia.add_adjacent(alemania_id);
        alemania.add_adjacent(polonia_id);
        brit.add_adjacent(alemania_id);
        alemania.add_adjacent(brit_id);
        let mut francia = Country::new(None, Continent::Europe, CName::Francia);
        let francia_id = francia.id();
        alemania.add_adjacent(francia_id);
        francia.add_adjacent(alemania_id);
        let mut españa = Country::new(None, Continent::Europe, CName::España);
        let españa_id = españa.id();
        francia.add_adjacent(españa_id);
        españa.add_adjacent(francia_id);
        brit.add_adjacent(españa_id);
        españa.add_adjacent(brit_id);
        let mut italia = Country::new(None, Continent::Europe, CName::Italia);
        let italia_id = italia.id();
        italia.add_adjacent(francia_id);
        francia.add_adjacent(italia_id);
        alemania.add_adjacent(italia_id);
        italia.add_adjacent(alemania_id);
        let mut sahara = Country::new(None, Continent::Africa, CName::Sahara);
        let sahara_id = sahara.id();
        sahara.add_adjacent(españa_id);
        españa.add_adjacent(sahara_id);
        bra.add_adjacent(sahara_id);
        sahara.add_adjacent(bra_id);
        let mut egipto = Country::new(None, Continent::Africa, CName::Egipto);
        let egipto_id = egipto.id();
        egipto.add_adjacent(sahara_id);
        sahara.add_adjacent(egipto_id);
        polonia.add_adjacent(egipto_id);
        egipto.add_adjacent(polonia_id);
        let mut etiopia = Country::new(None, Continent::Africa, CName::Etiopía);
        let etiopia_id = etiopia.id();
        etiopia.add_adjacent(egipto_id);
        egipto.add_adjacent(etiopia_id);
        sahara.add_adjacent(etiopia_id);
        etiopia.add_adjacent(sahara_id);
        let mut zaire = Country::new(None, Continent::Africa, CName::Zaire);
        let zaire_id = zaire.id();
        zaire.add_adjacent(etiopia_id);
        etiopia.add_adjacent(zaire_id);
        sahara.add_adjacent(zaire_id);
        zaire.add_adjacent(sahara_id);
        let mut sudafrica = Country::new(None, Continent::Africa, CName::Sudáfrica);
        let sudafrica_id = sudafrica.id();
        sudafrica.add_adjacent(zaire_id);
        zaire.add_adjacent(sudafrica_id);
        etiopia.add_adjacent(sudafrica_id);
        sudafrica.add_adjacent(etiopia_id);
        let mut madagascar = Country::new(None, Continent::Africa, CName::Madagascar);
        let madagascar_id = madagascar.id();
        zaire.add_adjacent(madagascar_id);
        madagascar.add_adjacent(zaire_id);
        egipto.add_adjacent(madagascar_id);
        madagascar.add_adjacent(egipto_id);
        let mut arabia = Country::new(None, Continent::Asia, CName::Arabia);
        let arabia_id = arabia.id();
        let mut aral = Country::new(None, Continent::Asia, CName::Aral);
        let aral_id = aral.id();
        rusia.add_adjacent(aral_id);
        aral.add_adjacent(rusia_id);
        let mut china = Country::new(None, Continent::Asia, CName::China);
        let china_id = china.id();
        let mut india = Country::new(None, Continent::Asia, CName::India);
        let india_id = india.id();
        china.add_adjacent(india_id);
        india.add_adjacent(china_id);
        let mut iran = Country::new(None, Continent::Asia, CName::Irán);
        let iran_id = iran.id();
        china.add_adjacent(iran_id);
        iran.add_adjacent(china_id);
        aral.add_adjacent(iran_id);
        iran.add_adjacent(aral_id);
        rusia.add_adjacent(iran_id);
        iran.add_adjacent(rusia_id);
        india.add_adjacent(iran_id);
        iran.add_adjacent(india_id);
        let mut tartaria = Country::new(None, Continent::Asia, CName::Tartaria);
        let tartaria_id = tartaria.id();
        aral.add_adjacent(tartaria_id);
        tartaria.add_adjacent(aral_id);
        let mut taymyr = Country::new(None, Continent::Asia, CName::Taymyr);
        let taymyr_id = taymyr.id();
        tartaria.add_adjacent(taymyr_id);
        taymyr.add_adjacent(tartaria_id);
        let mut japon = Country::new(None, Continent::Asia, CName::Japón);
        let japon_id = japon.id();
        china.add_adjacent(japon_id);
        japon.add_adjacent(china_id);
        let mut kamchatka = Country::new(None, Continent::Asia, CName::Kamchatka);
        let kamchatka_id = kamchatka.id();
        china.add_adjacent(kamchatka_id);
        kamchatka.add_adjacent(china_id);
        japon.add_adjacent(kamchatka_id);
        kamchatka.add_adjacent(japon_id);
        alaska.add_adjacent(kamchatka_id);
        kamchatka.add_adjacent(alaska_id);

        let mut siberia = Country::new(None, Continent::Asia, CName::Siberia);
        let siberia_id = siberia.id();
        kamchatka.add_adjacent(siberia_id);
        siberia.add_adjacent(kamchatka_id);
        china.add_adjacent(siberia_id);
        siberia.add_adjacent(china_id);
        taymyr.add_adjacent(siberia_id);
        siberia.add_adjacent(taymyr_id);
        tartaria.add_adjacent(siberia_id);
        siberia.add_adjacent(tartaria_id);
        aral.add_adjacent(siberia_id);
        siberia.add_adjacent(aral_id);
        let mut mongolia = Country::new(None, Continent::Asia, CName::Mongolia);
        let mongolia_id = mongolia.id();
        china.add_adjacent(mongolia_id);
        mongolia.add_adjacent(china_id);
        siberia.add_adjacent(mongolia_id);
        mongolia.add_adjacent(siberia_id);
        iran.add_adjacent(mongolia_id);
        mongolia.add_adjacent(iran_id);
        aral.add_adjacent(mongolia_id);
        mongolia.add_adjacent(aral_id);
        let mut gobi = Country::new(None, Continent::Asia, CName::Gobi);
        let gobi_id = gobi.id();
        mongolia.add_adjacent(gobi_id);
        gobi.add_adjacent(mongolia_id);
        china.add_adjacent(gobi_id);
        gobi.add_adjacent(china_id);
        iran.add_adjacent(gobi_id);
        gobi.add_adjacent(iran_id);
        let mut malasia = Country::new(None, Continent::Asia, CName::Malasia);
        let malasia_id = malasia.id();
        china.add_adjacent(malasia_id);
        malasia.add_adjacent(china_id);
        india.add_adjacent(malasia_id);
        malasia.add_adjacent(india_id);
        let mut turquia = Country::new(None, Continent::Asia, CName::Turquía);
        let turquia_id = turquia.id();
        iran.add_adjacent(turquia_id);
        turquia.add_adjacent(iran_id);
        rusia.add_adjacent(turquia_id);
        turquia.add_adjacent(rusia_id);
        polonia.add_adjacent(turquia_id);
        turquia.add_adjacent(polonia_id);
        egipto.add_adjacent(turquia_id);
        turquia.add_adjacent(egipto_id);
        arabia.add_adjacent(turquia_id);
        turquia.add_adjacent(arabia_id);
        let mut israel = Country::new(None, Continent::Asia, CName::Israel);
        let israel_id = israel.id();
        turquia.add_adjacent(israel_id);
        israel.add_adjacent(turquia_id);
        arabia.add_adjacent(israel_id);
        israel.add_adjacent(arabia_id);
        egipto.add_adjacent(israel_id);
        israel.add_adjacent(egipto_id);
        let mut sumatra = Country::new(None, Continent::Oceania, CName::Sumatra);
        let sumatra_id = sumatra.id();
        india.add_adjacent(sumatra_id);
        sumatra.add_adjacent(india_id);
        let mut borneo = Country::new(None, Continent::Oceania, CName::Borneo);
        let borneo_id = borneo.id();
        malasia.add_adjacent(borneo_id);
        borneo.add_adjacent(malasia_id);
        let mut java = Country::new(None, Continent::Oceania, CName::Java);
        let java_id = java.id();
        let mut australia = Country::new(None, Continent::Oceania, CName::Australia);
        let australia_id = australia.id();
        java.add_adjacent(australia_id);
        australia.add_adjacent(java_id);
        borneo.add_adjacent(australia_id);
        australia.add_adjacent(borneo_id);
        sumatra.add_adjacent(australia_id);
        australia.add_adjacent(sumatra_id);
        chile.add_adjacent(australia_id);
        australia.add_adjacent(chile_id);
        // Israel
        let res = hashmap! {
            {arg_id:arg},
            {chile_id:chile},
            {uru_id:uru},
            {bra_id:bra},
            {peru_id:peru},
            {col_id:col},
            {mex_id:mex},
            {cal_id:cal},
            {ny_id:ny},
            {ore_id:ore},
            {alaska_id:alaska},
            {yukon_id:yukon},
            {canada_id:canada},
            {terranova_id:terranova},
            {labrador_id:labrador},
            {gro_id:gro},
            {islandia_id:islandia},
            {brit_id:brit},
            {suecia_id:suecia},
            {rusia_id:rusia},
            {polonia_id:polonia},
            {alemania_id:alemania},
            {francia_id:francia},
            {españa_id:españa},
            {italia_id:italia},
            {sahara_id:sahara},
            {egipto_id:egipto},
            {etiopia_id:etiopia},
            {zaire_id:zaire},
            {sudafrica_id:sudafrica},
            {madagascar_id:madagascar},
            {arabia_id:arabia},
            {aral_id:aral},
            {china_id:china},
            {india_id:india},
            {iran_id:iran},
            {tartaria_id:tartaria},
            {taymyr_id:taymyr},
            {japon_id:japon},
            {kamchatka_id:kamchatka},
            {siberia_id:siberia},
            {mongolia_id:mongolia},
            {gobi_id:gobi},
            {malasia_id:malasia},
            {turquia_id:turquia},
            {israel_id:israel},
            {sumatra_id:sumatra},
            {borneo_id:borneo},
            {java_id:java},
            {australia_id:australia},
        };
        res
    })
    .unwrap();
    
}
// pub static MISSIONS: LazyLock<HashMap<&'static str, Mission>> = LazyLock::new(||{
//     // let m1 = "Conquistar Asia y América del Sur";
//     // let _f1 = Mission { name: m1, exe: Arc::new(|player_id, status|{
//     //     status.iter().all(|st|{
//     //         let country = MAP.get(st.0).unwrap();
//     //         country.continent() != Continent::Asia && country.continent() != Continent::SouthAmerica || st.1.tokens.as_ref().map_or(false, |t| t.owner == player_id)
//     //     })
//     // }) };
//     // let m2 = "Conquistar Asia y África";
//     // let _f2 = Mission { name: m2, exe: Arc::new(|player_id,status|{
//     //     status.iter().all(|st|{
//     //         let country = MAP.get(st.0).unwrap();
//     //         country.continent() != Continent::Asia && country.continent() != Continent::Africa || st.1.tokens.as_ref().map_or(false, |t| t.owner == player_id)
//     //     })
//     // }) };
//     // let m3 = "Conquistar América del Norte y África";
//     // let _f3 = Mission { name: m3, exe: Arc::new(|player_id,status|{
//     //     status.iter().all(|st|{
//     //         let country = MAP.get(st.0).unwrap();
//     //         country.continent() != Continent::NorthAmerica && country.continent() != Continent::Africa || st.1.tokens.as_ref().map_or(false, |t| t.owner == player_id)
//     //     })
//     // }) };
//     /*
// Conquistar América del Norte y Oceanía
// Conquistar Europa y América del Sur
// Conquistar Europa y Oceanía
// Conquistar Europa y África
// Conquistar Asia y Europa
// Conquistar Oceanía y América del Sur */
//     HashMap::new()
// });
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Move {
    player_id: u32,
    move_type: MoveType,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MoveType {
    Attack {
        from: Country,
        to: Country,
        troops: u32,
    },
    Fortify {
        from: Country,
        to: Country,
        troops: u32,
    },
    Pass,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveResult {
    success: bool,
    attacker: MoveResultCount,
    defender: MoveResultCount,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MoveResultCount {
    Win(u32),
    Lose(u32),
    Draw,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Country {
    pub id: Uuid,
    continent: Continent,
    name: CName,
    adjacents: Vec<Uuid>,
}
impl Country {
    pub fn new(id: Option<Uuid>, continent: Continent, name: CName) -> Self {
        Country {
            id: id.unwrap_or_else(Uuid::new_v4),
            continent,
            name,
            adjacents: Vec::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> CName {
        self.name
    }

    pub fn continent(&self) -> Continent {
        self.continent
    }

    pub fn add_adjacent(&mut self, adjacent: Uuid) {
        self.adjacents.push(adjacent);
    }

    pub fn adjacents(&self) -> &[Uuid] {
        &self.adjacents
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum Continent {
    #[default]
    NorthAmerica,
    SouthAmerica,
    Europe,
    Asia,
    Africa,
    Oceania,
}

impl Iterator for Continent {
    type Item = Continent;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Continent::NorthAmerica => Some(Continent::SouthAmerica),
            Continent::SouthAmerica => Some(Continent::Europe),
            Continent::Europe => Some(Continent::Asia),
            Continent::Asia => Some(Continent::Africa),
            Continent::Africa => Some(Continent::Oceania),
            Continent::Oceania => None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum CName {
    Canadá,
    Yukón,
    Alaska,
    Groenlandia,
    Oregón,
    California,
    México,
    NuevaYork,
    Terranova,
    Labrador,
    Argentina,
    Brasil,
    Perú,
    Colombia,
    Chile,
    Uruguay,
    GranBretaña,
    Islandia,
    España,
    Francia,
    Alemania,
    Italia,
    Polonia,
    Rusia,
    Suecia,
    Sahara,
    Etiopía,
    Egipto,
    Madagascar,
    Zaire,
    Sudáfrica,
    Arabia,
    Aral,
    China,
    India,
    Irán,
    Tartaria,
    Taymyr,
    Japón,
    Kamchatka,
    Siberia,
    Mongolia,
    Gobi,
    Malasia,
    Turquía,
    Israel,
    Sumatra,
    Borneo,
    Java,
    Australia,
}

impl Display for CName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CName::Canadá => "Canadá",
            CName::Yukón => "Yukón",
            CName::Alaska => "Alaska",
            CName::Groenlandia => "Groenlandia",
            CName::Oregón => "Oregón",
            CName::California => "California",
            CName::México => "México",
            CName::NuevaYork => "NuevaYork",
            CName::Terranova => "Terranova",
            CName::Labrador => "Labrador",
            CName::Argentina => "Argentina",
            CName::Brasil => "Brasil",
            CName::Perú => "Perú",
            CName::Colombia => "Colombia",
            CName::Chile => "Chile",
            CName::Uruguay => "Uruguay",
            CName::GranBretaña => "GranBretaña",
            CName::Islandia => "Islandia",
            CName::España => "España",
            CName::Francia => "Francia",
            CName::Alemania => "Alemania",
            CName::Italia => "Italia",
            CName::Polonia => "Polonia",
            CName::Rusia => "Rusia",
            CName::Suecia => "Suecia",
            CName::Sahara => "Sahara",
            CName::Etiopía => "Etiopía",
            CName::Egipto => "Egipto",
            CName::Madagascar => "Madagascar",
            CName::Zaire => "Zaire",
            CName::Sudáfrica => "Sudáfrica",
            CName::Arabia => "Arabia",
            CName::Aral => "Aral",
            CName::China => "China",
            CName::India => "India",
            CName::Irán => "Irán",
            CName::Tartaria => "Tartaria",
            CName::Taymyr => "Taymyr",
            CName::Japón => "Japón",
            CName::Kamchatka => "Kamchatka",
            CName::Siberia => "Siberia",
            CName::Mongolia => "Mongolia",
            CName::Gobi => "Gobi",
            CName::Malasia => "Malasia",
            CName::Turquía => "Turquía",
            CName::Israel => "Israel",
            CName::Sumatra => "Sumatra",
            CName::Borneo => "Borneo",
            CName::Java => "Java",
            CName::Australia => "Australia",
        })
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    id: Uuid,
    name: String,
    mission: String,
    countries: Vec<Uuid>,
    role: PlayerRole,
    adding_troops: u8,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Copy, Debug)]
pub enum PlayerRole {
    Master,
    Player { room: Uuid },
}
impl Player {
    pub fn new(name: String, role: PlayerRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            mission: String::new(),
            countries: Vec::new(),
            role,
            adding_troops: 0,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mission(&self) -> &str {
        &self.mission
    }

    pub fn countries(&self) -> &[Uuid] {
        &self.countries
    }

    pub fn role(&self) -> PlayerRole {
        self.role
    }

    pub fn set_mission(&mut self, mission: String) {
        self.mission = mission;
    }

    pub fn set_countries(&mut self, countries: Vec<Uuid>) {
        self.countries = countries;
    }

    pub fn available_troops(&self) -> u8 {
        self.adding_troops
    }

    pub fn grant_troops(&mut self, troop_count: u8) {
        self.adding_troops += troop_count;
    }
}
impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("role", &self.role)
            .field("countries", &self.countries)
            .field("available_troops", &self.adding_troops)
            // .field("mission", &self.mission)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tokens {
    pub owner: Uuid,
    pub amount: u8,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageDTO {
    AddPlayer {
        player: Player,
    },
    MakeMove {
        room_id: Uuid,
        player_id: Uuid,
        from: Uuid,
        to: Uuid,
        troops: u32,
    },
    StartGame {
        room_id: Uuid,
    },
    MissionCompleted {
        room_id: Uuid,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ResponseDTO {
    CompleteUpdate {
        room: RoomMaster,
        this_player: Uuid,
        players: HashMap<Uuid, Player>,
        status: HashMap<Uuid, Tokens>,
    },
    UpdateState {
        statuses: HashMap<Uuid, Tokens>,
    },
    UpdateRoom {
        room_id: Uuid,
        players: HashMap<Uuid, Player>,
        statuses: HashMap<Uuid, Tokens>,
    },
    GameStarted {
        starter: Uuid,
        players: HashMap<Uuid, Player>,
        missions: HashMap<Uuid, String>,
        status: HashMap<Uuid, Tokens>,
    },
    LoggedIn {
        this_player: Player,
        room: RoomMaster,
        users: HashMap<Uuid, Player>,
    },
    MissionCompleted {
        player: Uuid,
    },
    Error {
        message: String,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RoomMaster {
    pub room_id: Uuid,
    pub master: Uuid,
}
// #[derive(Serialize, Deserialize)]
// pub struct Mission<'a> {
//     name: &'a str,
//     objective: Option<Uuid>,
//     exe: Arc<dyn Fn(Uuid,HashMap<Uuid, Tokens>) -> bool + Send + Sync>,
// }
