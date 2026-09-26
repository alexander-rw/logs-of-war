# Logs of War

A team battle game: soldiers on two teams fight on a chosen map.

## Language

**Soldier**:
One log fighter in a battle. It has a personal name, belongs to one team, and has health.
_Avoid_: Character, tree character, log soldier, unit

**Team**:
One of the two sides in a battle: Red or Blue. Each soldier belongs to exactly one team.
_Avoid_: Team id, side, faction

**Player-controlled team**:
The team on a map whose soldiers the keyboard drives. A map has one such team or none.
_Avoid_: Human team, local player, controlled team

**Health**:
How much damage a soldier can still take. A soldier with no health is dead and leaves the battle.
_Avoid_: HP, hit points

**Map**:
A battlefield the player can pick: its terrain plus where each team spawns and which team the player controls.
_Avoid_: Level, arena, map terrain

**Terrain**:
The ground of one map: its shape and the surface soldiers stand on. Each map has exactly one terrain.
_Avoid_: Ground, landscape, level geometry

**Map selection**:
The map the player picked on the briefing screen for the next battle. It stays picked until the player changes it.
_Avoid_: Selected level

**Formation**:
A layout of spawn positions for one team: a line of soldiers at even spacing.
_Avoid_: Spawn group, team layout

**Battle**:
One fight between the two teams on the map selection, from the briefing's begin until it ends.
_Avoid_: Game, match

**Briefing**:
The screen before a battle where the player picks the map selection and begins the battle.
_Avoid_: Pre-game, map settings
