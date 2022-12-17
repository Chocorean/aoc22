# Advent of Code 2022

My modest attempt at AoC22 in Rust.

## Comments

### After validating day 13, Dec 17

Overall, I'm quite disappointed with my understanding of the problems. Either I
don't read well, or my implementations are missing a case, or at least I don't
think enough while writing.
I've lost a ton of time looking for an error while I
just did not understand what the game was expecting.

A little bit disappointed at myself, but at least now that I realized some of my
weaknesses, that makes room for improvement.
I want to use `clippy` when I will be done with this year's AoC, to see what I
can improve with the code I currently write. I hope I will have the motivation
to try the previous editions of AoC, and I hope to notice a difference with this
year's code.

Finally, I found very helpful to look at [fasterthanlime's serie](https://fasterthanli.me/series/advent-of-code-2022).
Not only he is using relevant and helpful I don't know about, but also has a
better knowledge of the language, and I have found some really convenient way to
do what I want in his snippets.

I have been cheating for the first time on day 13 (*bruh*), but I was starting
to lose confidence and I needed to move on. After verifying my implementation
was wrong (I validated part 1 with his code), I went back to my implementation
once again, reviwed my logic, and finally found what the [mistake](./c13/src/main.rs#L39).