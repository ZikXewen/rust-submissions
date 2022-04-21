# SPOJ - METEORS

## Statement

- `N` states, `M` sectors
- Each sector has 1 owner state.
- Each state has a target number.
- `Q` actions, range increment. (if `l > r`, increment `l..M` and `1..r`)
- Calculate number of action after which each state reach their target.

## Naive Approach

- For each state, binary search on actions. At least `O(NQ)`.

## Possible Solutions

- Partially Persistent Segment Tree

  - Generate `Q` versions of trees, adding actions one by one.
    - `O(Q * logM)`
  - For each state, binary search on actions with sum of every sector owned by that state.
    - `O(M * logM * logQ)`

- Parallel Binary Search

  - Define `l[]` and `r[]` for every state and set to `1` and `Q` respectively.
  - For each iteration, mark `mid` of each state.
  - Sweep through actions, adding them one by one (to either a Fenwick tree or a Segment tree).
    - `O(Q * logM)`
  - Every `mid[i]` we reach, query for the sum of every sector owned by `i`.

    ```
    if query(i) >= target[i]: r[i] = mid[i]
    else: l[i] = mid[i+1]
    ```

    - `O(M * logM)`

  - There will be `O(logQ)` iterations before every `l[i] == r[i]`

- BST-ish Implementation

  - View data as BST e.g.
    - First level consist of `(1, M)`.
    - Second level consist of `(1, M/2)` and `(M/2 + 1, M)`
    - ...
  - Iterate each level.
  - For each `(l, r, states)`, add action number `l` to `m`.

    ```
    for i in states:
        if query(i) >= target[i]: l_states.push(i)
        else: r_states.push(i)
    ```

  - Push `(l, m, l_states)` and `(m+1, r, r_states)` to next iteration.
  - Roughly `O((M+Q) * logM * logQ)`

## Notes

- Some implementations might exceed `long long int`, `unsigned long long int` is recommended.
- More on the final implementation [here](https://codeforces.com/blog/entry/45578?#comment-301605).
