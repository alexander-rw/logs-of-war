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

### Summary

- Replace placeholder cuboids with proper `LogCharacter` entities (capsule/cylinder mesh)
- `Team` component (`TeamId`, `TeamColor`)
- Spawn 2 teams x 4 characters each, positioned on opposite sides of the map
- Replace flat cylinder with static terrain mesh (simple heightmap or tiled box terrain)
- Health bar UI above each character (world-space billboard)

### 1.1 Team Component & Data Model

**File:** `src/components/team.rs`

```rust
/// Identifies which team a character belongs to.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Team {
    pub id: TeamId,
}

/// Unique identifier for each team (up to 2 teams for MVP).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TeamId {
    Red,
    Blue,
}

impl TeamId {
    /// Returns the team's display color for meshes and UI.
    pub fn color(&self) -> Color {
        match self {
            TeamId::Red => Color::srgb(0.85, 0.2, 0.2),   // crimson red
            TeamId::Blue => Color::srgb(0.2, 0.4, 0.85), // royal blue
        }
    }

    /// Returns a human-readable team name.
    pub fn name(&self) -> &'static str {
        match self {
            TeamId::Red => "Red Team",
            TeamId::Blue => "Blue Team",
        }
    }
}
```

**Integration:**
- Add `pub mod team;` to `src/components/mod.rs`
- Update `LogCharacter` bundle spawning to include `Team` component

### 1.2 Log Soldier Character Model

**File:** `src/log_character/log_character.rs` (extend existing)

Replace the placeholder cuboid with a "log soldier" visual:

- **Body:** Vertical cylinder (radius 0.3, height 1.2) — the log torso
- **Head:** Sphere (radius 0.25) positioned atop the cylinder
- **Collider:** Capsule collider (radius 0.3, height 1.2) for physics

**New helper function:**

```rust
/// Spawns a log soldier entity with mesh, collider, and components.
///
/// # Arguments
/// * `commands` - Bevy command buffer
/// * `meshes` - Mesh asset store
/// * `materials` - Material asset store
/// * `position` - World position to spawn at
/// * `team` - Team assignment
///
/// # Returns
/// The spawned entity ID.
pub fn spawn_log_soldier(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    team: TeamId,
) -> Entity {
    let body_mesh = meshes.add(Cylinder::new(0.3, 1.2));
    let head_mesh = meshes.add(Sphere::new(0.25));
    let material = materials.add(StandardMaterial {
        base_color: team.color(),
        ..default()
    });

    // Parent entity: the body cylinder with physics
    let parent = commands
        .spawn((
            Mesh3d(body_mesh),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(position),
            RigidBody::Dynamic,
            Collider::capsule(0.3, 1.2),
            LogCharacter::generate(),
            Team { id: team },
        ))
        .id();

    // Child entity: the head sphere (visual only, no collider)
    commands.spawn((
        Mesh3d(head_mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(0.0, 0.85, 0.0)),
    )).set_parent(parent);

    parent
}
```

**Notes for Python developers:**
- `commands.spawn(...)` returns an `EntityCommands` builder; `.id()` extracts the raw `Entity` handle.
- `.set_parent(parent)` establishes a transform hierarchy — the head moves with the body.
- `Collider::capsule(radius, height)` creates a pill-shaped collider ideal for bipedal characters.

### 1.3 Team Spawning System

**File:** `src/systems/spawn_teams.rs`

```rust
/// Spawns two teams of 4 log soldiers each on opposite sides of the map.
///
/// Team Red spawns at negative X, Team Blue at positive X.
/// Characters are spaced 2 units apart along the Z axis.
pub fn spawn_teams(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const TEAM_SIZE: usize = 4;
    const TEAM_X_OFFSET: f32 = 8.0;
    const SPAWN_HEIGHT: f32 = 2.0;
    const Z_SPACING: f32 = 2.0;

    // Calculate Z offset to center the team formation
    let z_start = -((TEAM_SIZE as f32 - 1.0) * Z_SPACING) / 2.0;

    for i in 0..TEAM_SIZE {
        let z = z_start + (i as f32) * Z_SPACING;

        // Red team (negative X)
        spawn_log_soldier(
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(-TEAM_X_OFFSET, SPAWN_HEIGHT, z),
            TeamId::Red,
        );

        // Blue team (positive X)
        spawn_log_soldier(
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(TEAM_X_OFFSET, SPAWN_HEIGHT, z),
            TeamId::Blue,
        );
    }
}
```

**Integration:**
- Add to `LogsOfWarPlugin` as `OnEnter(GameState::Game)` system
- Remove the old placeholder cube/cuboid spawning code from `game_setup`

### 1.4 Terrain Mesh

**File:** `src/systems/spawn_terrain.rs`

Replace the flat cylinder floor with a proper terrain:

**Option A: Flat tiled box terrain (simpler)**

```rust
/// Spawns a flat rectangular terrain with tiled visual appearance.
pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_mesh = meshes.add(Cuboid::new(40.0, 0.5, 40.0));
    let terrain_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.2), // grass green
        perceptual_roughness: 0.9,
        ..default()
    });

    commands.spawn((
        Mesh3d(terrain_mesh),
        MeshMaterial3d(terrain_material),
        Transform::from_translation(Vec3::new(0.0, -0.25, 0.0)),
        RigidBody::Static,
        Collider::cuboid(40.0, 0.5, 40.0),
    ));
}
```

**Option B: Simple heightmap terrain (more visual interest)**

For a basic heightmap, generate a plane mesh with perturbed Y values:

```rust
/// Generates a heightmap terrain mesh with gentle hills.
///
/// # Arguments
/// * `size` - Total width/depth of terrain
/// * `subdivisions` - Number of quads per axis (higher = smoother)
/// * `height_scale` - Maximum hill height
fn generate_heightmap_mesh(size: f32, subdivisions: u32, height_scale: f32) -> Mesh {
    // Use Bevy's Plane as base, then modify vertices
    let mut mesh = Plane3d::default()
        .mesh()
        .size(size, size)
        .subdivisions(subdivisions)
        .build();

    // Perturb Y values with simple sine-based hills
    if let Some(positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        if let VertexAttributeValues::Float32x3(positions) = positions {
            for pos in positions.iter_mut() {
                // Simple procedural height: overlapping sine waves
                let height = (pos[0] * 0.1).sin() * (pos[2] * 0.15).cos() * height_scale;
                pos[1] = height;
            }
        }
    }

    // Recalculate normals after vertex modification
    mesh.compute_normals();
    mesh
}
```

**Collider consideration:**
- For Option B, use `Collider::trimesh_from_mesh(&mesh)` to generate a physics collider matching the visual geometry.
- This is more expensive than a cuboid but necessary for non-flat terrain.

### 1.5 Despawn System Refinement

**Current state:** `despawn_character` in `src/systems/despawn_on_should_despawn_true.rs` already despawns entities when `!log_character.is_alive()`.

**Improvements needed:**
- The system works but should also handle recursive despawning (head child entity)
- Add a death animation/effect hook (future milestone)

```rust
/// Despawns log soldiers when their health reaches zero.
///
/// Uses `despawn_recursive` to also remove child entities (head mesh).
pub fn despawn_dead_characters(
    mut commands: Commands,
    query: Query<(Entity, &LogCharacter)>,
) {
    for (entity, character) in query.iter() {
        if !character.is_alive() {
            info!("{} has been eliminated!", character.name);
            commands.entity(entity).despawn_recursive();
        }
    }
}
```

**Note:** Replace `despawn()` with `despawn_recursive()` to clean up child entities (the head sphere).

### 1.6 Health Bar UI (World-Space Billboard)

**File:** `src/ui/health_bar.rs`

Health bars should float above each character and always face the camera.

**Components:**

```rust
/// Tag component linking a health bar to its owner entity.
#[derive(Component)]
pub struct HealthBar {
    pub owner: Entity,
}

/// Marker for the filled portion of the health bar.
#[derive(Component)]
pub struct HealthBarFill;
```

**Spawning health bars:**

```rust
/// Spawns a world-space health bar above a log soldier.
///
/// The health bar consists of:
/// - Background quad (dark gray, full width)
/// - Fill quad (team color, width proportional to health)
pub fn spawn_health_bar(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    owner: Entity,
    team: TeamId,
) -> Entity {
    const BAR_WIDTH: f32 = 0.8;
    const BAR_HEIGHT: f32 = 0.08;
    const BAR_Y_OFFSET: f32 = 1.4; // Above the head

    let bg_mesh = meshes.add(Rectangle::new(BAR_WIDTH, BAR_HEIGHT));
    let fill_mesh = meshes.add(Rectangle::new(BAR_WIDTH, BAR_HEIGHT));

    let bg_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.2),
        unlit: true, // No lighting for UI elements
        ..default()
    });

    let fill_material = materials.add(StandardMaterial {
        base_color: team.color(),
        unlit: true,
        ..default()
    });

    // Background
    let bar_entity = commands
        .spawn((
            Mesh3d(bg_mesh),
            MeshMaterial3d(bg_material),
            Transform::from_translation(Vec3::new(0.0, BAR_Y_OFFSET, 0.0)),
            HealthBar { owner },
        ))
        .id();

    // Fill (slightly in front to prevent z-fighting)
    commands.spawn((
        Mesh3d(fill_mesh),
        MeshMaterial3d(fill_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.01)),
        HealthBarFill,
    )).set_parent(bar_entity);

    bar_entity
}
```

**Billboard system (make health bars face camera):**

```rust
/// Rotates health bars to always face the camera.
pub fn billboard_health_bars(
    camera_query: Query<&Transform, With<Camera3d>>,
    mut bar_query: Query<&mut Transform, (With<HealthBar>, Without<Camera3d>)>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };

    for mut bar_transform in bar_query.iter_mut() {
        // Face the camera by copying its rotation
        bar_transform.rotation = camera_transform.rotation;
    }
}
```

**Health bar update system:**

```rust
/// Updates health bar fill width based on character health.
pub fn update_health_bars(
    character_query: Query<&LogCharacter>,
    bar_query: Query<(&HealthBar, &Children)>,
    mut fill_query: Query<&mut Transform, With<HealthBarFill>>,
) {
    for (bar, children) in bar_query.iter() {
        let Ok(character) = character_query.get(bar.owner) else {
            continue; // Owner despawned
        };

        // Health percentage (0.0 to 1.0)
        let health_pct = (character.health as f32 / 100.0).clamp(0.0, 1.0);

        // Update fill scale
        for child in children.iter() {
            if let Ok(mut fill_transform) = fill_query.get_mut(*child) {
                fill_transform.scale.x = health_pct;
                // Offset to keep left-aligned
                fill_transform.translation.x = -0.4 * (1.0 - health_pct);
            }
        }
    }
}
```

### 1.7 Camera Adjustments

Update `update_camera` in `logs_of_war.rs` to frame all characters:

```rust
/// Positions the camera to view the entire battlefield.
pub fn update_camera(
    mut camera_query: Query<&mut Transform, (With<Camera3d>, With<GameCamera>)>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    // Elevated position looking down at center of battlefield
    camera_transform.translation = Vec3::new(0.0, 15.0, 20.0);
    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}
```

### 1.8 Module Organization

**New files to create:**
- `src/components/team.rs` — Team component and TeamId enum
- `src/systems/spawn_teams.rs` — Team spawning system
- `src/systems/spawn_terrain.rs` — Terrain spawning system
- `src/ui/mod.rs` — UI module root
- `src/ui/health_bar.rs` — Health bar components and systems

**Files to modify:**
- `src/components/mod.rs` — Export team module
- `src/systems/mod.rs` — Export new systems
- `src/log_character/log_character.rs` — Add `spawn_log_soldier` helper
- `src/plugins/logs_of_war.rs` — Wire up new systems, remove placeholder spawns

### 1.9 System Scheduling

Add to `LogsOfWarPlugin::build()`:

```rust
// Startup systems (run once on entering Game state)
app.add_systems(
    OnEnter(GameState::Game),
    (spawn_terrain, spawn_teams).chain(),
);

// Update systems (run every frame while in Game state)
app.add_systems(
    Update,
    (
        billboard_health_bars,
        update_health_bars,
        update_camera,
    )
    .run_if(in_state(GameState::Game)),
);

// Fixed update systems (physics tick)
app.add_systems(
    FixedUpdate,
    despawn_dead_characters.run_if(in_state(GameState::Game)),
);
```

### 1.10 Testing Checklist

- [ ] Two teams of 4 characters spawn on opposite sides
- [ ] Characters are cylinder+sphere "log soldiers" with team colors
- [ ] Terrain is visible and characters stand on it via physics
- [ ] Health bars appear above each character
- [ ] Health bars billboard toward camera
- [ ] Damaging a character updates its health bar
- [ ] Characters despawn (with head) when health reaches 0
- [ ] Camera frames the entire battlefield
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo test` passes

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
