use dioxus::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{value::Value, from_value};
use regex::Regex;

pub fn logic(cx: Scope) -> Element {
   let hull_encode: HashMap<String, usize> = HashMap::from([
      ("Gunship".to_string(), 0),
      ("Escort".to_string(), 1),
      ("Corvette".to_string(), 2),
      ("Frigate".to_string(), 3),
      ("Monitor".to_string(), 4),
      ("Destroyer".to_string(), 5),
      ("Cruiser".to_string(), 6),
      ("Battlecruiser".to_string(), 7),
      ("Battleship".to_string(), 8),
      ("Lancer".to_string(), 9),
      ("Dreadnought".to_string(), 10),
      ("Titan".to_string(), 11),
      ("AlienGunship".to_string(), 12),
      ("AlienCorvette".to_string(), 13),
      ("AlienFrigate".to_string(), 14),
      ("AlienDestroyer".to_string(), 15),
      ("AlienBattlecruiser".to_string(), 16),
      ("AlienBattleship".to_string(), 17),
      ("AlienDreadnought".to_string(), 18),
      ("AlienAssaultCarrier".to_string(), 19),
      ("AlienMothership".to_string(), 20),
   ]);

   let mut grid: Vec<GridData> = Vec::new();
   let hull = use_state(&cx, || "Gunship".to_string());
   let templates = crate::template::template();
   let ship: ShipData = from_value(templates[0][hull_encode[hull.as_str()]].to_owned()).unwrap();
   
   let re: Regex = Regex::new(r"(Armor)").unwrap();
   for i in &ship.shipModuleSlots {
      let mut temp = i.moduleSlotType.as_str();
      if re.is_match(temp) {
         temp = temp.get(re.find(temp).unwrap().range()).unwrap();
      }
      grid.push(GridData {
         image: format!("http://127.0.0.1:8080/icons/shipbuildericons/empty_{}.png", temp),
         x: i.x,
         y: i.y
      })
   }

   crate::console_log!("from logic: {}", hull);
   cx.render(rsx! {
      crate::frontend::frontend {grid: grid, hull: hull.clone()}
   })
}

#[derive(PartialEq, Clone, Debug)]
pub struct GridData {
   pub image: String,
   pub x: i32,
   pub y: i32
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShipData{
   dataName: String,
   friendlyName: String,

   noseHardpoints: i32,
   hullHardpoints: i32,
   internalModules: i32,

   length_m: i32,
   width_m: i32,
   volume: i32,
   thrusterMultiplier: i32,
   structuralIntegrity: i32,
   mass_tons: i32,
   crew: i32,
   alien: bool,
   monthlyIncome_Money: i32,
   missionControl: i32,

   requiredProjectName: String,
   weightedBuildMaterials: HashMap<String, Value>,
   baseConstructionTime_days: i32,
   shipModuleSlots: Vec<ShipModuleData>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShipModuleData {
   moduleSlotType: String,
   x: i32,
   y: i32,
}

