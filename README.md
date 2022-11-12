# ThreadedBogoMergeSort (TBMSort)

Takes a list of natural numbers and sorts them using ThreadedBogoMergeSort.

Fun fact: a sorting algorithm called BogoSort shuffles a list randomly until it
becomes sorted; it is $O(n\cdot n!)$ where n is the size of the sequence,
and '!' is the postfix unary operator for factorial. BogoSort is often cited as
a slowest sorting algorithm.

Fun fact 2: TBMSort is multithreaded AND *slower* than BogoSort.

## What is TBMSort

### Context: BogoSort

Here is some pseudocode for bogosort

```
fn bogosort(rng, arr)
    if is_sorted(arr)
        arr
    else
        (rng_, arr_) = shuffle(rng, arr)
        bogosort(rng_, arr_)
```

(You can't just ignore the random number generator.)

### Context: MergeSort

Here is some pseudocode for mergesort

```
fn mergesort(arr)
    if length(arr) <= 1
        arr
    else
        n = (length(arr) / 2) as int
        l_result = mergesort(arr[..n])
        r_result = mergesort(arr[n..])
        merge(l_result, r_result)
```

The merge step in the original mergesort have the precondition that the two
input arrays are already sorted, say, least to greatest; suppose the head
elements of the arrays can be popped off on a one-by-one basis, then the
precondition would guarantee that each head element is $\le$ all following
elements in the same array, and the least of heads of the two arrays will be
the minimum of all elements of the two arrays. Thus we can take out the minimum
of the two arrays with only 1 comparison between a total of 2 numbers. The
merge step takes this minimum repeatedly and put it at the *back* of the return
array (since all existing elements in the return array are less than all
remaining elements in the two input arrays) until there are no elements left
from the inputs.

### Getting funky: BogoMergeSort

Here is some pseudocode for BogoMergeSort

```
fn bmsort(rng, arr)
    if length(arr) <= 1
        arr
    else
        n = (length(arr) / 2) as int
        (rng, l_result) = bmsort(rng, arr[..n])
        (rng, r_result) = bmsort(rng, arr[n..])
        maybe_result = shuffle(rng, join(l_result, r_result))
        if is_sorted(maybe_result)
            maybe_result
        else
            bmsort(rng, maybe_result)
```

As you can see, it is basically bogosort, except it redoes all lower-level
shuffles. Where upon one "run" the chance for bogosort to complete is $1/n!$,
it is $P(n)$ for BMSort where
```math
\begin{align*}
    P : \mathbb N &\to \mathbb R \\
    0 &\mapsto 1 \\
    1 &\mapsto 1 \\
    n &\mapsto \frac1{n!}
        \cdot P\left(\left\lceil\frac n2\right\rceil\right)
        \cdot P\left(\left\lfloor\frac n2\right\rfloor\right)
\end{align*}
```

TBMSort is the threaded version of BogoMergeSort, performing the
divide-and-conquer in parallel with threads. For every recursion in the D&C, it
spawns a new thread and uses the thread-local RNG provided by the `rand` crate.
The end result is that it makes full use of your CPU *while being slow*.

## CLI

usage:

```
tbmsort NATURALS...
```

(where `NATURALS` is actually `usize`.)
