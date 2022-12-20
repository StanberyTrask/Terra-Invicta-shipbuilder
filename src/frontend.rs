use dioxus::prelude::*;
//use serde_json::Value;

use crate::logic::GridData;

//the scale factor of the ship grid in px
static GRID_SCALE: f32 = 256f32 * 0.3f32;

#[inline_props]
pub fn frontend(cx: Scope, grid: Vec<GridData>, hull: UseState<String>) -> Element {
   crate::console_log!("from frontent: {}", hull);

   cx.render(rsx!{
      div {
         style: 
            "left: 0px;
            top: 0px;
            width: calc({GRID_SCALE}px * 10);
            height: calc({GRID_SCALE}px * 4.5);",

         grid.iter().map(|name| rsx!(
            div {
               style: 
                  "position: absolute;
                  left: calc({GRID_SCALE}px  * {name.x});
                  top: calc({GRID_SCALE}px  * calc({name.y} * 0.5));
                  width: {GRID_SCALE}px;
                  height: {GRID_SCALE}px;
                  user-select: none;",
               img {
                  src: "{name.image}",
                  draggable: "false",
                  width: "100%", height: "100%"
               }
            }
            
         ))
      }
      div {
         style: 
         "height: 22px;
         width: calc({GRID_SCALE}px * 10);
         display: flex;
         justify-content: space-between;
         flex-direction: row;",
         div {
            input {id: "ship-name", name: "ship-name",}
         }
         div {
            select {
               oninput: move |evt| {
                  hull.set(evt.value.clone());
                  //cx.needs_update();
               },
               name: "Ship Hull",
               id: "ship-hull",
               option {value: "Gunship", "Gunship"}
               option {value: "Escort", "Escort"}
               option {value: "Corvette", "Corvette"}
               option {value: "Frigate", "Frigate"}
               option {value: "Monitor", "Monitor"}
               option {value: "Destroyer", "Destroyer"}
               option {value: "Cruiser", "Cruiser"}
               option {value: "Battlecruiser", "Battlecruiser"}
               option {value: "Battleship", "Battleship"}
               option {value: "Lancer", "Lancer"}
               option {value: "Dreadnought", "Dreadnought"}
               option {value: "Titan", "Titan"}
            }
         }
         div {
            select {
               name: "Ship Role",
               id: "ship-role",
               option {value: "Troop Carrier", "Troop Carrier"}
               option {value: "Explorer", "Explorer"}
               option {value: "Inner System Colony Ship", "Inner System Colony Ship"}
               option {value: "Outer System Colony Ship", "Outer System Colony Ship"}
               option {value: "Transport", "Transport"}
               option {value: "Strike", "Strike"}
               option {value: "Interdictor", "Interdictor"}
               option {value: "Attack Bomber", "Attack Bomber"}
               option {value: "Fighter", "Fighter"}
               option {value: "Space Superiority", "Space Superiority"}
               option {value: "Standoff", "Standoff"}
               option {value: "Interceptor", "Interceptor"}
               option {value: "Patrol", "Patrol"}
               option {value: "Defense Bomber", "Defense Bomber"}
            }
         }
            
      }
      div {
         style:
            "position: relative;
            left: 0px;
            bottom: 0px;
            width: calc({GRID_SCALE}px * 10);
            height: calc(100% - 22px - calc({GRID_SCALE}px * 4.5));
            color: white;",
         h2 {"Ship Data"}
         p {"Wet Mass"}
         p {"Crew"}
         p {"Cruise Acceleration"}
         p {"Combat Acceleration"}
         p {"Cruise Delta-V"}
         p {"Turn Rate"}
         p {"Heat Sink Capacity"}
         p {"Construction Cost"}
         p {"Construction Time"}
         p {"Support"}
      }
   })
}