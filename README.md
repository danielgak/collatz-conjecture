## Collatz conjecture

Let's see where I can get trying to brute force the search of a counterexample of the conjecture. There exist alogorithms capable of hitting really big numbers like [2^100000-1](https://ieeexplore.ieee.org/document/8560077), but copying it's not my goal. 

My goal it's to go through the rationalle, and iteratevely play and improve with rus, to gain performance while having fun.

## V0 - base

Let's implement the simpliest representation of the problem as a **base to compare against**.

Having `base_loop = 5` and `x = base_loop`
```
while x is bigger or equal than base do:
    - x/2 if 2%0 = 0
    - else 3x+1 
    - repeat while x > base_loop
if the loop stops, number is checked
increment base_loop by 1 and repeat
```
This basically assumes that starting the loop (4, 2, 1) every number that already was checked, will get into the same result as the operations are deterministic.

So without any other optimization it's easy to check that every single number follows the conjecture up to `10_000_000_000`. But as I started with u64 with precision `(2^64-1) 18,446,744,073,709,551,615` I would have been very happy proving up to this number. But ofcourse, I forgot that x grows much more than the base_loop value.
That's why when checking `12_327_829_503` the x overflows at some point and the code panics, ups.

## V1 - u120
