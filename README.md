# My answers for Advent of Code 2022

My solutions for the AoC 2022 puzzles in Rust. 

## Day 13
Left unsolved. I knew what I had to do but it was too hard to implement and took too much time.
- use recursive enum for Packet
- parse with recursive function
- implement Ord trait for Packet

Solutions from [Reddit](https://www.reddit.com/r/adventofcode/comments/zkmyh4/2022_day_13_solutions/): https://github.com/julianandrews/adventofcode/blob/master/2022/rust/src/bin/day13.rs
https://pastebin.com/6H6HpES1


# Day 15
Part 2 left unsolved.
- Apparantly bruteforcing would've worked in Rust.
- Part 2 was completely different from part 1, so rewriting everything would've taken too much time.


# Day 16
Found I had to use Graph algorithm like Dijkstra, or A*.
- Pathfinding crate: decent option, but a bit too limited for customized graphs
- Petgraph crate: seems more customizeable but also more complex, especially if you have no experience with it.
- Finding a heuristic for A* was going to be very hard

# Day 17
Part 2 unsolved.
- the simulation is cyclic. Wasn't very clear to me on how to implement the solution.
