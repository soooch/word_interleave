# Word Interleave

### A small, fast, concurrent program with a really dumb goal

["What is my purpose?" "You find words composed of smaller interleaved words" "…oh my god"](https://www.youtube.com/watch?v=X7HmltUWXgs)

Some fun-ish interleaves:

- foe + old = fooled
- shoe + cold = schooled
- polls + epees = peopleless

While the results aren't particularly stunning, this was actually a pretty fun project. It started when a 
roommate found some words that interleaved in a moderately amusing manner. We decided it would be cool to 
find every instance of this "phenomenon".

Me and a friend quickly made naive solutions that took forever. We each built an O(n^2) algorithm which interleaved
each word with every other word and checked whether the result existed.

After finding that far too slow to reasonably wait on, I eventually landed on the idea of going backwards: I took each
word and deinterleaved it, then checked if the two deinterleaved words existed. This made the algorithm O(n)[^1].

Finally, I refactored the code into a declarative style, sprinkled in some Rayon, and got rid of some of my dumber
ideas like trying to keep separate dictionaries for words of different lengths.

[^1]: Or technically O(nlogn) if you count the sort at the end. In practice, since the sort operates on a set much
smaller than the original, it does not add much runtime.
