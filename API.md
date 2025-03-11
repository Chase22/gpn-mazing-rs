# client packets

| type  | parameters | response | description                                                                     |
|-------|------------|----------|---------------------------------------------------------------------------------|
| info  |            | info     | returns info about the maze                                                     |
| start |            | cell     | starts the game and returns the initial position of the player                  |
| get   |            | cell     | returns info about the current position                                         |
| move  | direction  | cell/end | moves the player in the given direction where direction is `N`, `E`, `S` or `W` |

# server packets

| type  | fields                    | description                                      |
|-------|---------------------------|--------------------------------------------------|
| info  | worldHeight worldWidth    | info about the maze                              |
| cell  | x y north east south west | all fields are integers where 1 indicates a wall |
| end   | moveCount                 | moves taken                                      |
| error | type message              | error message                                    |

# error types
| type               | description                          |
|--------------------|--------------------------------------|
| gameNotStarted     | game not started yet                 |
| gameAlreadyStarted | game already started                 |
| invalidMove        | invalid move, eg. moving into a wall |
| invalidDirection   | invalid direction, eg. `X`           |
| invalidPacket      | invalid packet                       |

