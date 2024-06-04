# Asteroids in Bevy

todo

- [x] implement https://paste.rs/7i9Ra.rs
- [x] Review PR
- [x] Shooting an asteroid turns it into 2 smaller asteroids
- [x] When asteroids spawn, they don't immediately kill the ship
- [x] When something dies, it explodes
- [x] Remove the explosions so we don't fill up memory if we play a long time
- [x] screens
  - [x] start
  - [x] get ready
  - [x] playing
  - [x] game over
  - [x] boss (when the boss walks in and we have to hide)
- [x] score
  - [x] track score in a resource
  - [x] display score on screen
  - [x] track high score in resource
  - [x] display high score
  - [x] save high score on game over
  - [x] load high score on startup
  - [x] update game over screen with correct score
  - [x] get ready should show correct high score
  - [x] when restarting game score should reset
- [x] After destroying all asteroids, level increases
  - [x] detect all asteroids are gone
  - [x] increase the number of asteroids
  - [x] switch to get ready state
- [x] teleport ship
- [x] rewrite collision system to use Rasmusgo's style
- [x] ufo
  - [x] spawn ufo x seconds into a level
  - [x] ufo moves randomly
  - [x] ufo fires bullets towards the current location of player
  - [x] ufo bullets can kill player
  - [x] ufo bullets can kill asteroids
  - [x] ship bullets can kill ufo
- [x] sounds
  - [x] firing bullet
  - [x] explosion
  - [x] ufo
  - [x] thrusting

# AI

- [ ] set speed of game to really fast
  - [ ] toggle speed on number key input
  - [ ] set speed
  - [ ] display speed
- [ ] toggle bot mode
- [ ] ai
  - [ ] loop
    - [ ] create population
    - [ ] for each individual
      - [ ] start a new game
      - [ ] play until it dies
    - [ ] next gen the bots
    - [ ] save the weights at the end of every gen
    - [ ] load weights from file 

  All sounds created by [chiptone](https://sfbgames.itch.io/chiptone).
