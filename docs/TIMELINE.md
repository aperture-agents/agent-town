# Project Timeline

We want to maintain a consistent steady development timeline which allows us to keep development fun, rewarding and production.

The proposed timeline is as follows:

## Phase 1 - Basic Game State ***

### Description

A Basic CLI Implementation of Mafia in which there exists:

- **Game State**: Controls the flow of the game and assignment of roles.
- **Players**: Who await their turn and can act based on their roles.
- **Chat**: Central chat system where players can interact and dispute.

### Acceptance Criteria

Should be able to play through a simple game of Mafia.

## Phase 2 - Agent Interface Adapation

### Description

Revise the basic implementation to get ready for agent tool interfacing:

- **Security**: Seperation of player memory and internal dialogue.
- **Tool Functions**: Player actions all via tool-like function interfaces.
- **State**: Maintained Player and Game history for agent to reference.

## Phase 3 - Graphic Interface

### Description

Voxel/Pixel style Mafia graphical interface:

- **Environment**: Render a 3d environment feature a roundtable, players and a nice town background.
- **Camera**: Scenery changes via a camera moving in the center of the roundtable.
- **Characters**: Rendered Characters with Animations for actions and dialogue.
- **Chat Box**: Chat box displayed for character dialogue, game events, and actions.

## Phase 4 - Agentic Implementation

### Description

Migrate player control over to agents via langgraph:

- **Tool Calls**: Agents will interact with their actions, role abilities, and memory via tool calls.
- **Models**: Should define multimodal support to power agent function.
- **Orchastration**: Ensure agents wait properly but can also interrupt eachother occasionally.
- **Concurrency**: Agents execute thinking in parallel to think in real time.

## Phase 5 - Character Customization

### Description

Allow for character sprite customization, lore, and personality:

- **Customization**: Allow for different outfits/body customization.
- **Lore**: Customize the characters backstory.
- **Personality**: Customize the characters personality and acts (Aggressive, Docile, etc)
