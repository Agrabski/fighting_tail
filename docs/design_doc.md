# The Fighting Tail

* Version: 0.4 *
* Date: August 24, 2025
* Author: AI-Assisted Design

## 1. Executive Summary

The Fighting Tail is a logistics-focused strategy game where the player assumes the role of a Division Logistics Officer. The game combines a lighthearted aesthetic (turtles vs. squirrels) with a deeply realistic and punishing set of logistics mechanics. The player's objective is to manage the supply chain, rear-area security, and C2 infrastructure for a single division, all while fulfilling the demanding and time-sensitive orders of the Division Commander. The core gameplay loop revolves around anticipating logistical needs, allocating limited resources, and reacting to dynamic threats to the supply chain.
## 2. Core Gameplay Loop

- Receive Orders: At the start of each in-game day, the player receives a new set of orders from the Division Commander. These orders are specific, time-sensitive, and directly related to the ongoing combat operation.

- Assess and Plan: The player reviews their current resources (trucks, MP platoons, engineers), checks supply stockpiles, and assesses the state of their C2 network. They then plan a course of action to fulfill the orders, allocating specific assets to specific tasks (e.g., sending a convoy, deploying an MP squad).

- Execute and React: The player watches their plan unfold on a hex-based map. Random events, such as ambushes, equipment breakdowns, or weather changes, will force the player to react and make critical decisions in real-time.

- Report and Evaluate: At the end of the day, the player sends a report to the Division Commander detailing their success or failure in meeting the day's objectives. This report affects the player's prestige, which serves as the primary progression system.

## 3. Game Mechanics
### 3.1 Map and Echelons

- Map: The game map is a hex grid representing a roughly 100km$^2$ area of operations, with each hex representing 1km$^2$. The map includes various terrain types (roads, forests, mountains, rivers) that affect movement speed and supply consumption.

- Echelons: The smallest manageable unit is a squad. Squads are organized into platoons, which form companies. The player manages units at all three levels of granularity, with the ability to detach individual squads or platoons for specific missions.

- Unit Stacking: Each hex has a limited stacking value (e.g., 6 points). Different unit types have different stacking values (e.g., Infantry Squad = 1, Vehicle Platoon = 2, Support Squad = 0.5). Exceeding the stacking limit results in penalties for all units in that hex (reduced C2, increased supply consumption).

### 3.2 Supply and Logistics

Supply Types: There are a variety of supply types, each with its own weight and value:

- Ammunition: Required for combat. Different unit types use different ammo (e.g., tank rounds, small arms).

- Fuel: Required for vehicles and power generators.

- Rations: Required for all units to maintain morale and combat effectiveness.

- Specialty Supplies: Medical supplies, vehicle parts, construction materials.

Supply Lines: The player creates and manages supply lines from rear-area depots to forward operating bases (FOBs) and directly to combat units. These are represented by a customizable route on the map that the player's truck convoys will follow.

Convoys: The core of the supply system. The player assembles convoys of different truck types (e.g., light, medium, armored) and assigns them a manifest and route. Convoys are vulnerable to ambushes and can break down.

FOBs: Forward Operating Bases are player-built intermediate supply hubs. They are crucial for pushing supplies to the front lines and provide a temporary C2 and repair base.

#### 3.2.1 Palletized vs. Fluid Logistics

The turtle army operates on a palletized logistics system. Supplies like ammunition, fuel, and rations are delivered from rear-area depots to FOBs and major command hubs on large, standardized pallets. This method is incredibly efficient for long-haul transport and is the fastest way to move bulk supplies.
However, these palletized supplies are not directly usable by frontline combat units. This creates a critical bottleneck that the player must manage. At FOBs, the player must dedicate manpower to break down pallets into smaller, usable containers. This conversion process is called transloading. It costs both time and manpower, typically requiring you to assign an MP or engineer platoon to an FOB for a set period. Without transloading, the supplies will simply sit at the FOB, useless to the fighting units. This forces the player to make critical trade-offs, deciding whether to use a limited MP platoon to secure a supply line or to speed up the critical transloading process.
In contrast, the enemy squirrels operate with more fluid logistics. Their supply lines are smaller and harder to hit, relying on faster, more adaptable transports carrying individual containers. This makes their logistics less efficient overall but far more resilient to the long-range attacks your army can mount.

## 3.3 Frontline Movement

The abstract "Prestige" score has been replaced with a tangible representation of success: the Frontline. The frontline is a dynamic, visible line on the map separating friendly and enemy-controlled territory. Its movement is directly influenced by your logistical support.

- Combat Effectiveness: Successful fire missions in support of a battalion’s assault will directly contribute to that battalion's Combat Effectiveness score (an invisible stat). This score increases with timely and sufficient fire support.

- Advancement: During the Execution phase of a turn, the game will check the combat effectiveness of each frontline unit. A higher score gives a higher probability of the frontline in that sector advancing one or more hexes.

- Stagnation or Retreat: If a battalion receives insufficient or no fire support, its combat effectiveness will decrease, and its frontline may fail to advance or could even retreat, signaling a stalemate or a setback.
- Your success is no longer a number—it's the forward march of your army.

## 3.4 Enemy Infiltration

Enemy forces pose a threat not only to the frontline but to your logistical supply lines. Enemy infiltration attempts are a risk in undefended areas and around unit flanks.

- Suppressive Fire: You can use your artillery batteries to conduct suppressive fire on a specific hex or group of hexes. This action temporarily reduces the chance of enemy infiltration in that area for the duration of the turn. This requires a dedicated fire mission and consumes ammunition.

- Consequences of Failure: If an infiltration attempt succeeds (because the hex was not suppressed), a small enemy unit will appear behind your lines. This unit will target your supply convoys and logistical assets, forcing you to divert resources to neutralize the threat and repair any damage.
- This mechanic introduces a critical defensive role for your artillery, turning it into a tool for area denial.

## 3.5 Counter-Battery Fire

Using your artillery is a strategic choice with inherent risks. The enemy will actively try to locate and neutralize your batteries.

Visibility: Every time one of your artillery batteries fires, its Visibility score increases. Firing repeatedly from the same location in a single turn will cause this score to rise rapidly.

The Strike: If an enemy counter-battery system is in range of your battery and your visibility score is too high, there is a chance the enemy will conduct a return fire mission during the Execution phase.

Outcomes: A successful counter-battery strike will have one of the following effects:

- Suppression: The battery is temporarily suppressed, reducing its fire rate and effectiveness for the rest of the turn.

- Damage: The battery is damaged, requiring a new logistical effort to bring engineers and spare parts for repairs.

- Relocation: The battery is forced to immediately relocate to a new position to avoid further strikes, creating a sudden logistical headache as you must reroute ammunition convoys to the new location.
- This creates a constant risk-reward dynamic, forcing you to balance the need for fire support with the threat of being hit by a counter-battery strike.

## 3.6 Casualty & Reinforcement Management

Units are no longer single entities but are composed of a number of personnel. This introduces a dual-mission logistical challenge.

- Personnel Count: A numerical value tied directly to a unit's Combat Effectiveness.

- Casualties: A separate count of personnel who are out of action. When casualties are incurred, the Personnel Count decreases, and the unit's effectiveness is reduced.

- Evacuating Wounded (CASEVAC): A time-sensitive mission requiring a dedicated medical vehicle to transport wounded personnel from the frontline to a medical facility.

- Providing Reinforcements (REINFORCEMENTS): A separate mission requiring a troop transport to deliver fresh personnel from a depot to a depleted unit to restore its strength.

## 3.7 Unit Reconstitution

A unit that has suffered heavy casualties cannot be effectively reinforced on the frontline. It must be pulled out of the fight entirely to rest and refit.

- The Threshold: If a unit's Personnel Count drops below a predetermined percentage of its starting strength (e.g., 50%), it is automatically flagged for reconstitution.

- Forced Retreat: The unit immediately loses its combat effectiveness and begins a forced retreat to the nearest friendly Rear Area. This retreat is a game-driven action that you cannot prevent.

- Rear Area: A Rear Area is a pre-designated or player-built location on the map, well behind the frontline, that is a safe zone with a reliable supply network.

- The Reconstitution Process: Once a unit reaches a Rear Area, it enters a "Reconstituting" status. This process takes a significant amount of in-game time (e.g., 2-3 days) and requires a steady flow of light supply. You can speed up the process by delivering reinforcements to the unit's new location.

- Strategic Consequences: The forced retreat of a unit creates a temporary, vulnerable gap in the frontline. This forces you to either have a reserve unit ready to move into the gap or risk an enemy breakthrough. It makes long-term unit sustainability a key strategic consideration.

## 3.8 Vehicle Maintenance and Repair

Every vehicle in a convoy will have a maintenance stat that slowly degrades with movement, especially over rough terrain. When a vehicle's maintenance falls too low, it has a high chance of breaking down, which can halt an entire convoy.

- Engineering Platoons: You will need to assign your limited engineering platoons to repair broken-down vehicles. Repairing a vehicle costs both time and specialized parts, a new supply type that you'll have to manage.

- Preventative Maintenance: You can choose to dedicate a vehicle to a scheduled maintenance stop at an FOB to prevent it from breaking down in the middle of a mission. This is a trade-off between speed and reliability.

## 3.9 Dynamic Weather and Terrain Impact

The map's terrain already affects movement, but a dynamic weather system will make it a constant factor. Every day could bring a different forecast, forcing you to plan your routes accordingly.

- Rain and Mud: Rain will turn dirt roads into mud, dramatically slowing convoys and increasing the risk of vehicle breakdowns. You might need to reroute your trucks or wait for the weather to clear, delaying a critical supply run.

- Snow and Ice: In colder climates, snow could make mountain passes impassable and increase fuel consumption for vehicles. You might have to allocate a snowplow asset to clear a key route, a job that would take a full day.

## 3.10 Civilian Factions and Guerrilla Activity

The "Civilian Morale" stat will be replaced by dynamic local factions. Each region will have a civilian population with an evolving relationship with your army.

- Hearts and Minds: You can drop "Goodwill Supplies" (extra rations, medical supplies) to a civilian population. This will increase their morale and make them more likely to share information about enemy movements and less likely to support guerrilla activity.

- Pacification: Alternatively, you can assign an MP platoon to a region to forcefully maintain order. This will quickly reduce unrest but also decrease civilian morale, which could lead to an uprising and the formation of hostile guerrilla cells. These guerrilla cells would then pose a direct threat to your supply lines.

## 3.11 Military Police (MP) & Rear Area Security

MP platoons are a key support asset. They can be assigned to:

- Convoy Escort: Provides a security bonus to a convoy, reducing the chance of ambush.

- Rear Area Patrol: Increases security and C2 in a specific hex or region, reducing the chance of sabotage and enemy infiltration.

- Traffic Control: Manages traffic on a congested supply line, reducing movement penalties.

- Morale Management: Increases the morale of units in an area, reducing the chance of desertion or insubordination.

- Enemy Sabotage: The opposing squirrel faction can deploy small, fast-moving special ops squads to target the player's supply lines, C2 network, and rear-area assets.

- Civilian Unrest: Some hexes may have a "Civilian Morale" stat. Low civilian morale can lead to local uprisings or insurgencies, which will directly impact the player's ability to move through the area. MPs can be used to pacify these areas.

## 3.12 C2 and Signals

    C2 Rating: Each hex has a C2 rating from 1 to 100%. This rating is affected by terrain, enemy jamming, and the presence of friendly command units and signals infrastructure.

    Command Units: A variety of units (Platoon HQ, Company HQ) act as mobile C2 hubs, radiating a signal to a certain radius of hexes.

    Signals Infrastructure: The player can build fixed assets like radio towers and relay stations to create a permanent, robust C2 network. These are a primary target for enemy sabotage.

    Loss of C2: Units outside of a C2 network suffer significant penalties to combat effectiveness, morale, and movement speed. Their orders are no longer dynamic, and they will only follow their last given order.

## 4. Art & Aesthetic

Theme
"Silly Graphics, Serious Mechanics."

World Design
The world should be visually appealing and clear. Hexes are clearly delineated, and terrain types are easily identifiable. Units are represented by small, recognizable icons that change based on their status (e.g., a broken-down vehicle icon, a panicked squad icon).

User Interface
The UI should be clean, professional, and easy to navigate. It should evoke the feel of a military operations center, with maps, status reports, and a streamlined command panel.

Character Design: Turtles
All turtles would have a similar base design: a classic, rounded shell, short legs, and a sturdy build that suggests durability and a slow, deliberate pace. Their expressions should be calm and determined, but not overly serious, reflecting their professional yet simple nature. They should be a clear visual contrast to the quick, agile squirrels.

Military Police (MP) Character Design
The MPs would be instantly recognizable by their distinct uniform and gear. Their design emphasizes their role as keepers of order and authority, with their iconic look making them stand out in any rear-area situation.

Uniform & Shell: Their military uniform is a deep crimson red, a classic MP color. Their helmets are a matching red, with a rounded shape that blends with their natural shell. The shell itself is a darker shade of green or brown, providing a natural, armored look.

Markings: A crisp white cross is painted on the front and back of their shell, making them highly visible for traffic control. They also wear a clean white web belt with a prominent, gold-colored badge on the chest. A crucial new detail is a bright white MP armband, worn on their left arm, clearly marking them as military police.

Gear: Each MP is equipped with a few key pieces of gear that visually communicate their purpose.

- Whistle & Clipboard: One hand is ready to blow a whistle, while the other is holding a clipboard. The clipboard is a simple yet effective way to show their administrative and regulatory duties.

- Traffic Baton: When directing traffic, they hold a short, brightly colored baton.

- Radio: A small, boxy radio is attached to their belt, showing their connection to the wider command network.

Expression: Their default expression is one of calm, patient professionalism. They aren't frontline fighters; they're there to maintain order. A slight, knowing squint shows their alertness without looking aggressive.

## 5. Target Audience

- Fans of strategic games who enjoy deep, complex mechanics (e.g., Hearts of Iron 4, Factorio, Dwarf Fortress).

- Players interested in the under-explored genre of logistics and supply chain management.

- Wargame enthusiasts looking for a new, more focused challenge.
