Flip game (Lights out, 点灯游戏、关灯游戏) typically plays on a 5x5 board. Pressing on any block (or light) will toggle itself and the 4 adjacent ones (might be less than 4 of near border). The final goal is to turn all blocks into On or Off state. 

https://en.wikipedia.org/wiki/Lights_Out_(game)

## Usage

Describe the board with chars in `input.txt`. Each char corresponds to one block on the board. Whitespace chars are ignored.

| Char | Block Type |
| - | - |
| 0 | Off |
| 1 | On |
| T/t | Border |
| G/g | Do not care |


All lines (after stripping all whitespaces) must have the same length.

See `input.txt` in the repo for example.

TODO:

- `G` blocks is not supported for now.
- Support multi answer boards.

## Simulate / Test

- Prepare `input.txt` as above. 
- Run `flipblard`, and copy the final answer to `ans.txt`
- Run `cat ans.txt | python3 sim.py`
- Check if output board is all zero.

Also you can play with the game:

- Prepare `input.txt` as above.
- Run `python3 sim.py`
- Type in your action, in form of `x,y`
- Type in `done` when finished.

Note that for simplicity the intermediate state of the board is hidden, which may not be suitable for actual playing. You might want to uncomment line 37 of `sim.py`.
