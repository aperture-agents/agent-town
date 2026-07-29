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
- sender: Sender { System | Player(id) }
- text: String
- phase: Option<GamePhase> this makes history self-describing, there's really no need for timestamps


