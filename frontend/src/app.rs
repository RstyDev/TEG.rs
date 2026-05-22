use std::{sync::Arc, collections::HashMap};
use macros::string;
use structs::{CName, CStatus, Map, Point};
use sycamore::prelude::*;
use crate::add_users::AddUsers;
const CSS: &str = "border-radius: 10px; border: 2px solid black; background-color: white; padding: 5px; font-size: 14px; font-weight: bold;";

#[component]
pub fn App() -> View {
    let map = Arc::new(Map::get());
    let users = create_signal(HashMap::new());
    let status = create_signal(map.0.clone().into_iter().map(|(country_id,c)|{
        (country_id,CStatus { country_id, location: get_point(c.name()), tokens: None })
    }).collect::<HashMap<_,_>>());
   
    create_memo(move || {
        console_log!("=-=-= Estado del juego: =-=-= {:?}", status.get_clone());
    });
    // console_dbg!(&map);

    view!{
        article(){
            img(src="./public/map.webp", alt="Mapa del juego",width="1200px", height="800px"){}
        }
        AddUsers(map=map, users=users, status=status)
        (status.get_clone().into_iter().map(|(_,c_status)|view!{
            article(class="tokens"){
                    p(style=format!("{}position:absolute; left:{}px; top:{}px;", CSS, c_status.location.x, c_status.location.y)){(match &c_status.tokens{
                        None => string!("0"),
                        Some(tokens) => format!("{}",tokens.amount),
                    })}
                }
        }).collect::<Vec<View>>())
        // Keyed(
        //     list=status,
        //     view=|c_status| view!{
        //         article(class="tokens"){
        //             p(style=format!("{}position:absolute; left:{}px; top:{}px;", CSS, c_status.location.x, c_status.location.y)){(match &c_status.tokens{
        //                 None => string!("0"),
        //                 Some(tokens) => format!("{}",tokens.amount),
        //             })}
        //         }
        //     },
        //     key=|c_status| c_status.id
        // )
    }
}

pub fn get_point(name: CName) -> Point {
    match name {
        CName::Canadá => Point{ x: 200, y: 150 },
        CName::Yukón => Point{ x: 120, y: 210 },
        CName::Alaska => Point{ x: 35, y: 290 },
        CName::Groenlandia => Point{ x: 425, y: 150 },
        CName::Oregón => Point{ x: 95, y: 350 },
        CName::California => Point{ x: 165, y: 385 },
        CName::México => Point{ x: 300, y: 415 },
        CName::NuevaYork => Point{ x: 215, y: 255 },
        CName::Terranova => Point{ x: 260, y: 240 },
        CName::Labrador => Point{ x: 320, y: 210 },
        CName::Argentina => Point{ x: 410, y: 545 },
        CName::Brasil => Point{ x: 450, y: 460 },
        CName::Perú => Point{ x: 360, y: 495 },
        CName::Colombia => Point{ x: 370, y: 430 },
        CName::Chile => Point{ x: 370, y: 600 },
        CName::Uruguay => Point{ x: 460, y: 540 },
        CName::GranBretaña => Point{ x: 660, y: 290 },
        CName::Islandia => Point{ x: 520, y: 280 },
        CName::España => Point{ x: 620, y: 420 },
        CName::Francia => Point{ x: 710, y: 360 }, 
        CName::Alemania => Point{ x: 770, y: 340 },
        CName::Italia => Point{ x: 760, y: 420 },
        CName::Polonia => Point{ x: 820, y: 330 },
        CName::Rusia => Point{ x: 815, y: 220 },
        CName::Suecia => Point{ x: 710, y: 200 },
        CName::Sahara => Point{ x: 720, y: 515 },
        CName::Etiopía => Point{ x: 805, y: 540 },
        CName::Egipto => Point{ x: 905, y: 530 },
        CName::Madagascar => Point{ x: 930, y: 600 },
        CName::Zaire => Point{ x: 755, y: 590 },
        CName::Sudáfrica => Point{ x: 860, y: 640 },
        CName::Arabia => Point{ x: 965, y: 465 },
        CName::Aral => Point{ x: 870, y: 170 },
        CName::China => Point{ x: 1050, y: 260 },
        CName::India => Point{ x: 1040, y: 390 },
        CName::Irán => Point{ x: 915, y: 280 },
        CName::Tartaria => Point{ x: 900, y: 135 },
        CName::Taymyr => Point{ x: 955, y: 135 },
        CName::Japón => Point{ x: 1150, y: 230 },
        CName::Kamchatka => Point{ x: 1030, y: 130 },
        CName::Siberia => Point{ x: 930, y: 205 },
        CName::Mongolia => Point{ x: 950, y: 240 },
        CName::Gobi => Point{ x: 1015, y: 275 },
        CName::Malasia => Point{ x: 1130, y: 390 },
        CName::Turquía => Point{ x: 875, y: 390 },
        CName::Israel => Point{ x: 875, y: 430 },
        CName::Sumatra => Point{ x: 990, y: 515 },
        CName::Borneo => Point{ x: 1080, y: 460 },
        CName::Java => Point{ x: 1135, y: 445 },
        CName::Australia => Point{ x: 1130, y: 545 },
    }
}
/*
OVERFLOW
Over 1: 15 34 56   
Over 2: 100 176
Prom: 78
New: 15

UNDERFLOW
Root: 
Leaf: 
Fusionar: 

NodosLibres: []

                                            2: [0](78)[1]
                                0: (34)(56)(78)(100)(176) 1: (100)(176)

LE 0: Overflow: divido en 15 34 56 | 78 100 176, promuevo 78 al nuevo nodo padre
E 1: 100 176
E 2: 78


*/