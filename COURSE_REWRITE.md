# FGJ course rewrite

> Concept: this rewrite is done for the purposes of
>
> 1) learning proper Rust syntax
> 2) gaining intuition on Rust
> 3) learning to apply optimization techniques

## code improvement

> Goal: The game works in under 150 lines of clearly written code.

### minimizing the code

- [ ] take out all clearly unused code.
- [ ] start simplifying the problem
- [ ] do one part of code at a time
- [ ] run the code frequently to see if the new version you wrote works.
- [ ] if the code's actual functionality (how it works) needs to be changed, don't be afraid to break everything. we can fix it.
- [ ] while minimizing code, insert rust language code features wherever possible.
  after understanding problems, try to minimize them using rust like features, whenever the code seems readable.

## game quality improvements

> Goal: get rid of jank

* [ ] get rid of the extra character appearing
* [ ] add a real-time updating timer that works as your score
* [ ] scale to fill the entire terminal (at launch) perhaps? probably not.

## gameplay improvements

> Goal: make the game more fun, whatever that means
- add varied snake types
- smarter snake spawning
- make difficulty scale with time survived
- make the game more responsive, if possible
- add something else for the character to do, perhaps
- add a health system
- add a trail animation or something to the player character

## optimization

> Goals:
> 1. make the gamecode at least 10 times faster. (shouldn't be hard)
> 2. if terminal interactions are the bottleneck
> - try to limit interfacing with it, especially drawing
> - try to find better ways of interfacing

#### Plan of action

* establish performance measurement
* try to balance game design with optimizations
* compare and contrast different parallel implementations

#### code isolation for measurement

> NOTE: isolating terminal calls and game logic from eachother is crucial

**Terminal**

* Just use the render and input functions.

**Game loop**

* find some way to time just the game loop in an optimized build
* disable terminal output or input
* disable death
* various snake amounts
* increase spawning rate
* if scaling makes no sense, then there are probably cache misses

### Performance oriented implementation

> NOTE: write parallel implementations and then compare performance

#### Data-structures

* struct-of-arrays data structure for gamestate:
  currently the game uses an array-of-structs
  [SOA](https://en.wikipedia.org/wiki/Data-oriented_design) is used for preventing CPU cache misses.
* game-specific array-based alternative to SOA:
  Array of 64 bit integers that gets bit shifted to move the snakes to the left
  would be crazy fast
  each row represented by an integer, each snake position represented by a bit set to 1. 

#### Other things to try

* try to compare and contrast copy vs reference
  * see that one c video for information. compare optimize flag assembly
* optimize construction of the frame buffer.
  * probably possible to change into branchless code.
  * verify branchlessness by lack of conditional jump commands in assembly
* try to apply the contents of these (ive already seen these):
  https://www.youtube.com/watch?v=lStYLF6Us_Q
  https://www.youtube.com/watch?v=w3_e9vZj7D8
