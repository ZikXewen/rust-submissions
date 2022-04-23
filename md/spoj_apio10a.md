# APIO10A - Commando

## Solution

Main concern: $dp_i = min(dp_j + f(sm(j + 1, i)))$

Naive dynamic programming approach with prefix sum will run in $O(n^2)$, which easily exceeds the time limit.

Let $sm_i$ be the prefix sum at position $i$. Therefore, $sm(j + 1, i) = sm_i - sm_j$

$$
\begin{align*}

dp_i & = min(dp_j + f(sm(j + 1, i)))\\
dp_i & = min(dp_j + f(sm_i - sm_j))\\
dp_i & = min(dp_j + a(sm_i - sm_j)^2 + b(sm_i - sm_j) + c)\\
dp_i & = min(dp_j + a \times sm_i^2 - 2 \times a \times sm_i \times sm_j + a \times sm_j^2 + b \times sm_i - b \times sm_j + c)\\
dp_i & = a \times sm_i^2 + b \times sm_i + c + min(-2 \times a \times sm_j \times sm_i + dp_j + a \times sm_j^2 - b * sm_j)\\
dp_i & = C + min(M_j \times sm_i + K_j)
\end{align*}
$$

Now that we can resolve the equation into this form, we can use [Convex Hull Trick](https://cp-algorithms.com/geometry/convex_hull_trick.html) or [Li Chao Tree](https://cp-algorithms.com/geometry/convex_hull_trick.html#li-chao-tree) to optimize our dynamic programming algorithm.

Thus reduces the time complexity to $O(n \log n)$
