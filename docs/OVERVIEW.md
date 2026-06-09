# Project Overview - AgentTown

## Goal

Utilize Aperture - Our custom stateful graph orchestration framework to pit AI Agents against one another in a classic game of Mafia.

## What is Mafia?

Mafia (also known as Werewolf in a popular variant) is a social deduction party game where a small group of hidden killers try to eliminate everyone else while avoiding detection.

### Core Roles
**Mafia**: Know who each other are. Each night they secretly choose a player to eliminate.
**Townspeople (Villagers)**: Do not know who the Mafia are. They must identify and vote out the Mafia during the day.

### Special Roles (optional):
**Detective/Investigator**: Can check a player's alignment each night.
**Doctor**: Can protect a player from being killed each night.

*Many variants add dozens of other roles.*

### Game Flow

**Night Phase**

Everyone closes their eyes.

The Mafia secretly choose someone to kill.

Special roles perform their actions (investigations, protections, etc.).

**Day Phase**

Everyone learns who died during the night.

Players discuss, accuse, defend themselves, and try to determine who is Mafia.

A vote is held, and one player is eliminated.

The eliminated player's role is usually revealed.

### Winning

Town wins when all Mafia members are eliminated.
Mafia wins when they reach parity with the town (they control enough votes that the town can no longer stop them).

### Why It's Fun

The game is less about mechanics and more about:

Bluffing
Reading body language and behavior
Creating alibis
Catching contradictions
Convincing others to trust you

*All Things AI completly SUCKS at - So all the more fun to watch.*

## Additional Resources

[Timeline](TIMELINE.md)
[Architecture](ARCHITECTURE.md)
[Contributing](../.github/CONTRIBUTING.md)
