# Logs of War — Development Plan

A spiritual successor to Hogs of War (PS1, 2000), rebuilt in Rust using Bevy + Avian3D.
Turn-based artillery strategy with log soldiers, destructible terrain, and faction campaigns.

## Current State

- Bevy 0.18 + Avian3D physics
- State machine: `Splash -> Menu -> Game`
- `LogCharacter` component (name, health, take_damage, is_alive)
- Placeholder game scene: floor + spinning cube + player cuboid
- 3s countdown then returns to menu
- Menu with settings (display quality)
- Save/load infrastructure via `bevy_pkv`

---

## Recommended Build Order

```
M1 (Teams/World) -> M2 (Turns) -> M3 (Movement) -> M4 (Bazooka) -> M5 (Win/Loss)
    -> M6 (Terrain) -> M7 (Weapons) -> M8 (Classes) -> M9 (Campaign) -> M10 (Polish)
```

Each milestone produces a playable slice of the game. M1-M5 is the MVP playable to a win condition.

---

## Milestone 1: Teams & World Foundation

**Goal:** Two teams of log soldiers exist in a real game world.

- Replace placeholder cuboids with proper `LogCharacter` entities (capsule/cylinder mesh)
- `Team` component (`TeamId`, `TeamColor`)
- Spawn 2 teams x 4 characters each, positioned on opposite sides of the map
- Replace flat cylinder with static terrain mesh (simple heightmap or tiled box terrain)
- Wire `ShouldDespawn` to `LogCharacter::is_alive()` — characters despawn at 0 health
- Health bar UI above each character (world-space billboard)

---

## Milestone 2: Turn System

**Goal:** The game takes turns between teams.

- `TurnManager` resource: tracks current team, current character index, turn number
- `TurnState` sub-state: `PlayerTurn`, `AITurn` (placeholder), `ResolvingAction`, `GameOver`
- Turn timer (30 seconds per turn) displayed in HUD
- At end of turn: advance to next character, then next team when all characters have gone
- HUD overlay: current team name, character name, timer countdown
- "End Turn" button / keybind

---

## Milestone 3: Character Movement

**Goal:** You can walk your active character around the terrain.

- Active character highlighted with an indicator ring
- `MovementBudget` component: movement units remaining this turn (reset each turn)
- WASD/arrow key movement translating character along terrain surface
- Terrain-following: downward raycast to stay on terrain slope
- Movement depletes budget; movement locked when budget exhausted
- Camera: 3rd-person follow camera orbiting active character (mouse drag to orbit, scroll to zoom)
- Visual indicator for movement range (projected circle on ground)

---

## Milestone 4: Aiming & First Weapon (Bazooka)

**Goal:** You can fire a projectile that deals damage.

- `Weapon` component and `WeaponKind` enum (start with `Bazooka`)
- Aim mode entered on fire key:
  - Left/right keys adjust heading angle
  - Up/down keys adjust elevation angle
  - Trajectory preview arc via Bevy gizmos
  - Charge bar for power (hold spacebar)
- On fire: spawn `Projectile` entity with `RigidBody::Dynamic` + `Collider`
- `ExplosionEvent`: on contact, deal damage to all `LogCharacter` entities within blast radius
- Camera tracks projectile in flight, returns to active character on landing
- Firing ends the current turn

---

## Milestone 5: Win/Loss & Game Loop

**Goal:** The game can be won and lost.

- After each explosion: check if any team has 0 living characters
- `GameOver` state: show "Victory!" / "Defeat!" screen with surviving team name
- Scoreboard showing characters remaining per team
- "Play Again" and "Main Menu" buttons
- Handle simultaneous multi-character death without crash

---

## Milestone 6: Terrain Deformation

**Goal:** Explosions reshape the battlefield.

- Replace static terrain with a voxel or heightmap-based deformable terrain
- `TerrainMap` resource: grid of height values
- On explosion: carve a sphere from terrain data, regenerate mesh in affected chunks
- Collider updated after deformation
- Characters fall into craters; falling off map edge is instant death

---

## Milestone 7: Weapon Arsenal

**Goal:** Multiple weapons with distinct behaviors and ammo limits.

| Weapon        | Behavior                                        |
|---------------|-------------------------------------------------|
| Bazooka       | Single shot, medium blast radius                |
| Grenade       | Thrown arc, 3s fuse, medium blast               |
| Cluster Bomb  | Splits into 5 bomblets mid-air                  |
| Airstrike     | Drops a horizontal cluster along a column       |
| Medpack       | Restores 25 HP to active character              |
| Dynamite      | Placed this turn, detonates next turn, large blast |

- `WeaponInventory` component: map of `WeaponKind -> ammo count`
- Weapon selection wheel (Tab to cycle; UI panel shows weapon + ammo)
- Per-weapon fire logic (fuse timer, split logic, placement, etc.)

---

## Milestone 8: Character Classes

**Goal:** Characters have roles that differentiate gameplay.

Mirroring Hogs of War's promotion system:

| Class          | Promotions Required | Notable Weapons                     |
|----------------|---------------------|-------------------------------------|
| Grunt          | 0 (default)         | Bazooka, Grenade                    |
| Medic          | 1                   | Medpack, Cluster Bomb               |
| Engineer       | 1                   | Dynamite, Airstrike, Rope           |
| Heavy Weapons  | 2                   | Holy Grenade, Big Nuke              |
| Espionage      | 2                   | Teleport, Disguise                  |
| Special Forces | 3                   | Full weapon access                  |

- `CharacterClass` component
- XP awarded for kills; promote on XP threshold
- Class-specific weapon loadouts applied on spawn/promotion

---

## Milestone 9: Factions & Campaign

**Goal:** Multiple factions with flavour and a campaign to play through.

Lumber-themed parody factions:

| Faction           | Inspiration |
|-------------------|-------------|
| Lumbershire       | Britain     |
| The Barksheviks   | Russia      |
| Splinter Stars    | USA         |
| Le Bois Garcons   | France      |
| Das Holzwerk      | Germany     |
| Moku-Tai          | Japan       |

- `Faction` resource with per-team assignment
- Campaign map screen: world map with mission nodes
- Per-mission config: opposing factions, terrain preset, squad composition
- XP and promotions carry over between missions
- Campaign progress saved/loaded via `bevy_pkv`

---

## Milestone 10: Polish & Feel

**Goal:** The game feels good to play.

- Character walk animation (procedural bobbing/rotation)
- Explosion particle effects (billboard sprites or custom particle system)
- Sound effects: weapon fire, explosion, character hit, death, water splash
- Background music per faction/map
- Narration flavour text on game events (kill, turn start, victory)
- Loading screen with asset progress indication
- Fullscreen/windowed toggle wired to settings menu
