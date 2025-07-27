# Sternhalma game server

## The Sternhalma game

The game is played on an hexagonal board in the shape of a six point star with
121 valid positions.
Each player has 15 pieces and start on the opposite sides of the board.
The pieces are represented by colored circles, one color for each player.

```text
   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠     󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   🔴
    󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠     󠀠󠀠󠀠󠀠   🔴 🔴
  󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠     🔴 🔴 🔴
   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠  🔴 🔴 🔴 🔴
   ⚫ ⚫ ⚫ ⚫ 🔴 🔴 🔴 🔴 🔴 ⚫ ⚫ ⚫ ⚫
  󠀠󠀠󠀠󠀠   ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫
   󠀠󠀠󠀠󠀠   ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫
       ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫
  󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫
   󠀠󠀠󠀠󠀠   ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫
    ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫
  ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫ ⚫
⚫ ⚫ ⚫ ⚫ 🔵 🔵 🔵 🔵 🔵 ⚫ ⚫ ⚫ ⚫
 󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   🔵 🔵 🔵 🔵
  󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   🔵 🔵 🔵
   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   🔵 🔵
    󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   󠀠󠀠󠀠󠀠   🔵
```

The game is turn based and a player can move one piece per turn.
A piece can be moved to an adjacent empty position or jump over an arbitrary
number of single pieces from any player.

The goal of the game is to move all one's pieces to the opposite side of the board.

## Architecture

