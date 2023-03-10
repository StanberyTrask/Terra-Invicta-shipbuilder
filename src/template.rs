use serde_json::{from_str, Value};

pub fn template() -> Vec<Value> {
   let mut out: Vec<Value> = Vec::new();

   
   let hull_str = 
   /* #region */
"[
   {
      \"dataName\": \"Gunship\",
      \"friendlyName\": \"Gunship\",
      \"noseHardpoints\": 1,
      \"hullHardpoints\": 0,
      \"internalModules\": 1,
      \"length_m\": 50,
      \"toylength_cm\": 7.1,
      \"width_m\": 10,
      \"volume\": 1963,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 3,
      \"mass_tons\": 200,
      \"crew\": 3,
      \"alien\": false,
      \"monthlyIncome_Money\": -1,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Gunship\",
         \"ships/Gunship_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_GS\",
         \"ui_spacecombat/OBJ_battle_earth_GS_ALT\"
      ],
      \"path1\": [
         \"earth_GS/\",
         \"earth_GS_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_GS\",
         \"OBJ_battle_earth_GS_ALT\"
      ],
      \"requiredProjectName\": \"Project_Warships\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 60,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 2,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 7,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 3
         }
      ]
   },
   {
      \"dataName\": \"Escort\",
      \"friendlyName\": \"Escort\",
      \"noseHardpoints\": 0,
      \"hullHardpoints\": 2,
      \"internalModules\": 2,
      \"length_m\": 50,
      \"toylength_cm\": 7.1,
      \"width_m\": 10,
      \"volume\": 1963,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 6,
      \"mass_tons\": 200,
      \"crew\": 4,
      \"alien\": false,
      \"monthlyIncome_Money\": -2,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Escort\",
         \"ships/Escort_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_ES\",
         \"ui_spacecombat/OBJ_battle_earth_ES_ALT\"
      ],
      \"path1\": [
         \"earth_ES/\",
         \"earth_ES_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_ES\",
         \"OBJ_battle_earth_ES_ALT\"
      ],
      \"requiredProjectName\": \"Project_Warships\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 90,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 2,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 7,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"Corvette\",
      \"friendlyName\": \"Corvette\",
      \"noseHardpoints\": 1,
      \"hullHardpoints\": 1,
      \"internalModules\": 2,
      \"length_m\": 65,
      \"toylength_cm\": 8.1,
      \"width_m\": 15,
      \"volume\": 7069,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 6,
      \"mass_tons\": 350,
      \"crew\": 8,
      \"alien\": false,
      \"monthlyIncome_Money\": -3,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Corvette\",
         \"ships/Corvette_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_CO\",
         \"ui_spacecombat/OBJ_battle_earth_CO_ALT\"
      ],
      \"path1\": [
         \"earth_CO/\",
         \"earth_CO_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_CO\",
         \"OBJ_battle_earth_CO_ALT\"
      ],
      \"requiredProjectName\": \"Project_Warships\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 90,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 7,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"Frigate\",
      \"friendlyName\": \"Frigate\",
      \"noseHardpoints\": 1,
      \"hullHardpoints\": 2,
      \"internalModules\": 4,
      \"length_m\": 100,
      \"toylength_cm\": 10,
      \"width_m\": 20,
      \"volume\": 23562,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 9,
      \"mass_tons\": 600,
      \"crew\": 20,
      \"alien\": false,
      \"monthlyIncome_Money\": -4,
      \"missionControl\": 2,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Frigate\",
         \"ships/Frigate_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_FRI\",
         \"ui_spacecombat/OBJ_battle_earth_FR_ALT\"
      ],
      \"path1\": [
         \"earth_FRI/\",
         \"earth_FR_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_FRI\",
         \"OBJ_battle_earth_FR_ALT\"
      ],
      \"requiredProjectName\": \"Project_PatrolVessels\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 135,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 7,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 5
         }
      ]
   },
   {
      \"dataName\": \"Monitor\",
      \"friendlyName\": \"Monitor\",
      \"noseHardpoints\": 0,
      \"hullHardpoints\": 4,
      \"internalModules\": 3,
      \"length_m\": 125,
      \"toylength_cm\": 11.2,
      \"width_m\": 20,
      \"volume\": 31416,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 9,
      \"mass_tons\": 700,
      \"crew\": 35,
      \"alien\": false,
      \"monthlyIncome_Money\": -4,
      \"missionControl\": 2,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Monitor\",
         \"ships/Monitor_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_MO\",
         \"ui_spacecombat/OBJ_battle_earth_MO_ALT\"
      ],
      \"path1\": [
         \"earth_MO/\",
         \"earth_MO_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_MO\",
         \"OBJ_battle_earth_MO_ALT\"
      ],
      \"requiredProjectName\": \"Project_PatrolVessels\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 135,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 7,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 3
         }
      ]
   },
   {
      \"dataName\": \"Destroyer\",
      \"friendlyName\": \"Destroyer\",
      \"noseHardpoints\": 2,
      \"hullHardpoints\": 2,
      \"internalModules\": 3,
      \"length_m\": 125,
      \"toylength_cm\": 11.2,
      \"width_m\": 20,
      \"volume\": 31416,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 12,
      \"mass_tons\": 700,
      \"crew\": 40,
      \"alien\": false,
      \"monthlyIncome_Money\": -5,
      \"missionControl\": 2,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Destroyer\",
         \"ships/Destroyer_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_DES\",
         \"ui_spacecombat/OBJ_battle_earth_DES_ALT\"
      ],
      \"path1\": [
         \"earth_DES/\",
         \"earth_DES_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_DES\",
         \"OBJ_battle_earth_DES_ALT\"
      ],
      \"requiredProjectName\": \"Project_PatrolVessels\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 135,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 7,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"Cruiser\",
      \"friendlyName\": \"Cruiser\",
      \"noseHardpoints\": 2,
      \"hullHardpoints\": 3,
      \"internalModules\": 6,
      \"length_m\": 175,
      \"toylength_cm\": 13.2,
      \"width_m\": 20,
      \"volume\": 47124,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 15,
      \"mass_tons\": 850,
      \"crew\": 60,
      \"alien\": false,
      \"monthlyIncome_Money\": -10,
      \"missionControl\": 3,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Cruiser\",
         \"ships/Cruiser_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_LC\",
         \"ui_spacecombat/OBJ_battle_earth_LC_ALT\"
      ],
      \"path1\": [
         \"earth_LC/\",
         \"earth_LC_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_LC\",
         \"OBJ_battle_earth_LC_ALT\"
      ],
      \"requiredProjectName\": \"Project_FleetCombatants\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 195,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 8,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 5
         }
      ]
   },
   {
      \"dataName\": \"Battlecruiser\",
      \"friendlyName\": \"Battlecruiser\",
      \"noseHardpoints\": 3,
      \"hullHardpoints\": 2,
      \"internalModules\": 4,
      \"length_m\": 175,
      \"toylength_cm\": 13.2,
      \"width_m\": 20,
      \"volume\": 47124,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 15,
      \"mass_tons\": 850,
      \"crew\": 70,
      \"alien\": false,
      \"monthlyIncome_Money\": -12,
      \"missionControl\": 3,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Battlecruiser\",
         \"ships/Battlecruiser_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_BC\",
         \"ui_spacecombat/OBJ_battle_earth_BC_ALT\"
      ],
      \"path1\": [
         \"earth_BC/\",
         \"earth_BC_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_BC\",
         \"OBJ_battle_earth_BC_ALT\"
      ],
      \"requiredProjectName\": \"Project_FleetCombatants\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 195,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 8,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 5
         }
      ]
   },
   {
      \"dataName\": \"Battleship\",
      \"friendlyName\": \"Battleship\",
      \"noseHardpoints\": 2,
      \"hullHardpoints\": 6,
      \"internalModules\": 5,
      \"length_m\": 200,
      \"toylength_cm\": 14.1,
      \"width_m\": 25,
      \"volume\": 85903,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 18,
      \"mass_tons\": 1150,
      \"crew\": 80,
      \"alien\": false,
      \"monthlyIncome_Money\": -15,
      \"missionControl\": 3,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Battleship\",
         \"ships/Battleship_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_BS\",
         \"ui_spacecombat/OBJ_battle_earth_BS_ALT\"
      ],
      \"path1\": [
         \"earth_BS/\",
         \"earth_BS_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_BS\",
         \"OBJ_battle_earth_BS_ALT\"
      ],
      \"requiredProjectName\": \"Project_FleetCombatants\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 195,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 0,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 0,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 8,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 3
         }
      ]
   },
   {
      \"dataName\": \"Lancer\",
      \"friendlyName\": \"Lancer\",
      \"noseHardpoints\": 4,
      \"hullHardpoints\": 3,
      \"internalModules\": 6,
      \"length_m\": 250,
      \"toylength_cm\": 15.8,
      \"width_m\": 32,
      \"volume\": 180956,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 21,
      \"mass_tons\": 1700,
      \"crew\": 100,
      \"alien\": false,
      \"monthlyIncome_Money\": -20,
      \"missionControl\": 4,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Lancer\",
         \"ships/Lancer_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_LAN\",
         \"ui_spacecombat/OBJ_battle_earth_LAN_ALT\"
      ],
      \"path1\": [
         \"earth_LAN/\",
         \"earth_LAN_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_LAN\",
         \"OBJ_battle_earth_LAN_ALT\"
      ],
      \"requiredProjectName\": \"Project_ShipsoftheLine\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 240,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 0,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 0,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 8,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 5
         }
      ]
   },
   {
      \"dataName\": \"Dreadnought\",
      \"friendlyName\": \"Dreadnought\",
      \"noseHardpoints\": 3,
      \"hullHardpoints\": 8,
      \"internalModules\": 6,
      \"length_m\": 275,
      \"toylength_cm\": 16.6,
      \"width_m\": 35,
      \"volume\": 240528,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 24,
      \"mass_tons\": 1950,
      \"crew\": 120,
      \"alien\": false,
      \"monthlyIncome_Money\": -25,
      \"missionControl\": 4,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Dreadnought\",
         \"ships/Dreadnought_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_DR\",
         \"ui_spacecombat/OBJ_battle_earth_DR_ALT\"
      ],
      \"path1\": [
         \"earth_DR/\",
         \"earth_DR_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_DR\",
         \"OBJ_battle_earth_DR_ALT\"
      ],
      \"requiredProjectName\": \"Project_ShipsoftheLine\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 240,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 0,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 0,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 9,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 9,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"Titan\",
      \"friendlyName\": \"Titan\",
      \"noseHardpoints\": 4,
      \"hullHardpoints\": 6,
      \"internalModules\": 8,
      \"length_m\": 300,
      \"toylength_cm\": 17.3,
      \"width_m\": 35,
      \"volume\": 264581,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 36,
      \"mass_tons\": 2050,
      \"crew\": 120,
      \"alien\": false,
      \"monthlyIncome_Money\": -30,
      \"missionControl\": 5,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/Titan\",
         \"ships/Titan_1\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_earth_TI\",
         \"ui_spacecombat/OBJ_battle_earth_TI_ALT\"
      ],
      \"path1\": [
         \"earth_TI/\",
         \"earth_TI_ALT/\"
      ],
      \"path2\": [
         \"OBJ_battle_earth_TI\",
         \"OBJ_battle_earth_TI_ALT\"
      ],
      \"requiredProjectName\": \"Project_Titans\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.7,
         \"nobleMetals\": 0.2,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 240,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 0,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 0,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 9,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 9,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"AlienGunship\",
      \"friendlyName\": \"Alien Gunship\",
      \"noseHardpoints\": 1,
      \"hullHardpoints\": 0,
      \"internalModules\": 2,
      \"length_m\": 50,
      \"toylength_cm\": 7.1,
      \"width_m\": 10,
      \"volume\": 1963,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 6,
      \"mass_tons\": 128,
      \"crew\": 1,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienGunship\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_GS\",
         \"\"
      ],
      \"path1\": [
         \"hydra_GS/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_GS\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.3,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 96,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 2,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 7,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"AlienCorvette\",
      \"friendlyName\": \"Alien Corvette\",
      \"noseHardpoints\": 1,
      \"hullHardpoints\": 1,
      \"internalModules\": 3,
      \"length_m\": 75,
      \"toylength_cm\": 8.7,
      \"width_m\": 10,
      \"volume\": 3927,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 9,
      \"mass_tons\": 192,
      \"crew\": 3,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienCorvette\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_CO\",
         \"\"
      ],
      \"path1\": [
         \"hydra_CO/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_CO\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.3,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 128,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 8,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"AlienFrigate\",
      \"friendlyName\": \"Alien Frigate\",
      \"noseHardpoints\": 1,
      \"hullHardpoints\": 3,
      \"internalModules\": 4,
      \"length_m\": 125,
      \"toylength_cm\": 11.2,
      \"width_m\": 15,
      \"volume\": 17671,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 12,
      \"mass_tons\": 384,
      \"crew\": 10,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienFrigate\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_FRI\",
         \"\"
      ],
      \"path1\": [
         \"hydra_FRI/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_FRI\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.3,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 256,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 8,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 3
         }
      ]
   },
   {
      \"dataName\": \"AlienDestroyer\",
      \"friendlyName\": \"Alien Destroyer\",
      \"noseHardpoints\": 2,
      \"hullHardpoints\": 2,
      \"internalModules\": 4,
      \"length_m\": 150,
      \"toylength_cm\": 12.2,
      \"width_m\": 15,
      \"volume\": 22089,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 18,
      \"mass_tons\": 448,
      \"crew\": 20,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienDestroyer\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_DES\",
         \"\"
      ],
      \"path1\": [
         \"hydra_DES/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_DES\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.3,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 360,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 7,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 5
         }
      ]
   },
   {
      \"dataName\": \"AlienBattlecruiser\",
      \"friendlyName\": \"Alien Battlecruiser\",
      \"noseHardpoints\": 3,
      \"hullHardpoints\": 3,
      \"internalModules\": 5,
      \"length_m\": 250,
      \"toylength_cm\": 15.8,
      \"width_m\": 25,
      \"volume\": 110447,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 24,
      \"mass_tons\": 1024,
      \"crew\": 35,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienBattlecruiser\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_BC\",
         \"\"
      ],
      \"path1\": [
         \"hydra_BC/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_BC\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.3,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 480,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 0,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 0,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 9,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 9,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 3
         }
      ]
   },
   {
      \"dataName\": \"AlienBattleship\",
      \"friendlyName\": \"Alien Battleship\",
      \"noseHardpoints\": 2,
      \"hullHardpoints\": 6,
      \"internalModules\": 6,
      \"length_m\": 250,
      \"toylength_cm\": 15.8,
      \"width_m\": 25,
      \"volume\": 110447,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 36,
      \"mass_tons\": 1024,
      \"crew\": 40,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienBattleship\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_BS\",
         \"\"
      ],
      \"path1\": [
         \"hydra_BS/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_BS\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.3,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 480,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 0,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 0,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 8,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"AlienDreadnought\",
      \"friendlyName\": \"Alien Dreadnought\",
      \"noseHardpoints\": 4,
      \"hullHardpoints\": 8,
      \"internalModules\": 8,
      \"length_m\": 300,
      \"toylength_cm\": 17.3,
      \"width_m\": 30,
      \"volume\": 194386,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 48,
      \"mass_tons\": 1344,
      \"crew\": 50,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienDreadnought\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_DN\",
         \"\"
      ],
      \"path1\": [
         \"hydra_DN/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_DN\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.3,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 540,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 0,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 0,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 9,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 9,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 4,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"AlienAssaultCarrier\",
      \"friendlyName\": \"Alien Assault Carrier\",
      \"noseHardpoints\": 0,
      \"hullHardpoints\": 6,
      \"internalModules\": 5,
      \"length_m\": 300,
      \"toylength_cm\": 17.3,
      \"width_m\": 30,
      \"volume\": 194386,
      \"thrusterMultiplier\": 1,
      \"structuralIntegrity\": 36,
      \"mass_tons\": 1344,
      \"crew\": 80,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienAssaultCarrier\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_AC\",
         \"\"
      ],
      \"path1\": [
         \"hydra_AC/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_AC\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.3,
         \"exotics\": null
      },
      \"baseConstructionTime_days\": 480,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 4,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 1,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 8,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 8,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 8,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 8,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 7,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 6,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 5,
            \"y\": 4
         }
      ]
   },
   {
      \"dataName\": \"AlienMothership\",
      \"friendlyName\": \"Alien Mothership\",
      \"noseHardpoints\": 4,
      \"hullHardpoints\": 16,
      \"internalModules\": 4,
      \"length_m\": 600,
      \"toylength_cm\": 24.5,
      \"width_m\": 60,
      \"volume\": 1625774,
      \"thrusterMultiplier\": 3,
      \"structuralIntegrity\": 96,
      \"mass_tons\": 3840,
      \"crew\": 100,
      \"alien\": true,
      \"monthlyIncome_Money\": 0,
      \"missionControl\": 1,
      \"shipyardyOffset\": [
         0,
         0,
         0
      ],
      \"modelResource\": [
         \"ships/AlienMothership\",
         \"\"
      ],
      \"combatUIpath\": [
         \"ui_spacecombat/OBJ_battle_hydra_MS\",
         \"\"
      ],
      \"path1\": [
         \"hydra_MS/\",
         \"\"
      ],
      \"path2\": [
         \"OBJ_battle_hydra_MS\",
         \"\"
      ],
      \"requiredProjectName\": \"Project_AlienMasterProject\",
      \"weightedBuildMaterials\": {
         \"volatiles\": 0.1,
         \"metals\": 0.6,
         \"nobleMetals\": 0.19,
         \"exotics\": 0.01
      },
      \"baseConstructionTime_days\": 640,
      \"shipModuleSlots\": [
         {
            \"moduleSlotType\": \"Drive\",
            \"x\": 0,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Power_Plant\",
            \"x\": 1,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Battery\",
            \"x\": 2,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Radiators\",
            \"x\": 3,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Tail_Armor\",
            \"x\": 0,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Lateral_Armor\",
            \"x\": 4,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Armor\",
            \"x\": 9,
            \"y\": 7
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 9,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 3
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Nose_Hard_Point\",
            \"x\": 8,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 0
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 6
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 4,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 7,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 6,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 2
         },
         {
            \"moduleSlotType\": \"Hull_Hard_Point\",
            \"x\": 5,
            \"y\": 4
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 3,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 3,
            \"y\": 5
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 2,
            \"y\": 1
         },
         {
            \"moduleSlotType\": \"Utility\",
            \"x\": 2,
            \"y\": 5
         }
      ]
   }
]";
/* #endregion */

   let hull = from_str(hull_str).unwrap();
   out.push(hull);

   return out;
}
