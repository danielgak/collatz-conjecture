## Collatz conjecture

Let's see where I can get trying to brute force the search of a counterexample of the conjecture. There exist alogorithms capable of hitting really big numbers like [2^100000-1](https://ieeexplore.ieee.org/document/8560077), but copying it's not my goal. 

My goal it's to go through the rationale, and iteratively play and improve with rust, to gain performance while having fun.

## V0 - base

Let's implement the simplest representation of the problem as a **base to compare against**.

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
That's why when checking `12_327_829_503` the x overflows at some point and the code panics.

## V1 - big int

Before I reached any performance bottleneck, I'm encountering that the first issue is storing and keeping efficiently in memory the value x.
I can use just double the precision (to u128), but that means postponing the problem for later.

Let's replace our x with a BigUint, and let's keep running.

Of course, bigUInt means that we have the actual value stored in the heap, 
and we that we are accessing memory basically in any operation, plus we need to clone the value few times on every iteration.

But that's a problem for another day... after few minutes of execution we surpassed the barrier and getting into the 13b (`12_482_780_605`).

## V2 - the another day

I've got 8 cpus, 7 of them watching how 1 struggles to get to 13b. [Rust atomic and locals](https://www.youtube.com/watch?v=99Qzpv325yI)

One thing that I notice is that V1 is noticeably slower than the simple V0, as BigUint no longer represent the number in a single register it seems to access memory more often.
To compare, I added criterion to be more precise about it:

```sh
cargo bench -- --release

open target/criterion/report/index.html 
```

And it shows that one step of V1 its ~70ns vs ~2ns for V0. That's something to worry. To understand where the time is spent, I'll use flamegraph.