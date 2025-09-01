# Demo

https://github.com/user-attachments/assets/5cf56a26-8672-461b-8842-8fa3b1f16aee

# Rustling

A 2D top-down action game where you fight enemies in a pixelated world. Built in Rust with a custom Entity Component System because apparently I enjoy making things harder for myself.

## What It Does

- **Custom ECS**: Built my own Entity Component System from scratch (with some unsafe Rust sprinkled in)
- **Enemy Combat**: Enemies chase you around and try to murder you
- **Map Editor Integration**: Levels designed in Tiled because drawing collision boxes by hand is for masochists
- **Dynamic Camera**: Follows the player without showing the dreaded void beyond the world
- **Animation System**: Sprites actually move their legs when walking

## Technical Bits

- Handles 1000+ entities before your computer starts crying (tested up to 1500 at 30 FPS)
- Strategic unsafe code for performance (yes, I know what I'm doing... mostly)

## Architecture

### Entity Component System
- **Components**: Position, Velocity, Sprite, Collider, Player, Enemy, etc.
- **Systems**: Movement, Animation, Rendering, Input, Combat, AI
- **World**: Central entity and component manager

### Key Systems
- **Movement System**: Handles physics and collision detection for all moveable entities
- **Animation System**: Manages sprite animations based on entity state and velocity
- **Enemy AI System**: Implements state machine for enemy behavior (wander/chase/attack/ded)
- **Combat System**: Handles player attacks and enemy damage with invincibility frames

## Controls

- **Arrow keys**: Move player
- **Z**: Attack

## Building and Running

```bash
# Clone the repository
git clone https://github.com/dylan0804/rustling
cd rustling

# Run the game
cargo run --release
```

## Project Structure

```
src/
├── main.rs             # Entry point and game loop
├── query.rs            # Component-related queries
├── world.rs            # ECS world and entity management
├── components/         # All game components
│   ├── mod.rs
│   ├── position.rs
│   ├── collider.rs
│   ├── velocity.rs
│   ├── direction.rs
│   ├── sprite.rs
│   ├── player.rs
│   └── enemy.rs
├── systems/             # Game systems
│   ├── mod.rs
│   ├── systems.rs
├── entity/              # Entity
│   ├── mod.rs
│   ├── entity.rs
└── resources.rs

assets/                  # Asset loading and management
├── core/                # Essential game assets
└── content/             # Gameplay content
```

## Assets

Game uses pixel art assets and Tiled maps. Place your assets in the `assets/` directory:
- Sprite sheets for characters and objects
- Tiled JSON map files
- Individual texture files for tilesets

## Future Improvements

- UI assets
- Enemy actually dealing damage
- Sound system integration
- Level transitions and multiple rooms
- Save/load functionality
