# agent-town chat api

something like:

```
  player or game -> send/system -> chat ->  append message to log
                                        |-> notify -> hooks/print/state?
```

we define MessageKind to be wither Speech or System

where Message:
- id: usize
- kind: MessageKind
- sender: Option<usize> which is Some(player_id) or none when System
- text: String
- phase: Option<GamePhase> this makes history self-describing, there's really no need for timestamps


