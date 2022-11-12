# ThreadedBogoMergeSort (TBMSort)

Takes a list of natural numbers and sorts them using ThreadedBogoMergeSort.

Fun fact: a sorting algorithm called BogoSort shuffles a list randomly until it
becomes sorted; it is $O(n\cdot n!)$ where n is the size of the sequence,
and '!' is the postfix unary operator for factorial. BogoSort is often cited as
a slowest sorting algorithm.

Fun fact 2: TBMSort is multithreaded AND *slower* than BogoSort.

## What is TBMSort

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

## CLI

usage:

```
tbmsort NATURALS...
```

(where `NATURALS` is actually `usize`.)
