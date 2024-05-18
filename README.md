# The Number Guessing "Paradox"

Based on the chapter "Pick the largest Number" by Thomas M. Cover in 1987's
"Open Problems in Communication and Computation":

> Player 1 writes down any two distinct numbers on separate slips of paper.
> Player 2 randomly chooses one of these slips of paper and looks at the number.
> Player 2 must decide whether the number in his hand is the larger of the two numbers.
> He can be right with a probability of one-half. It seems absurd that he can do better.
>
> We argue that Player 2 has a strategy by which he can correctly state whether
> or not the other number is larger or smaller than the number in his hand
> with probability _strictly greater than one-half_.
>
> **Solution:** The idea is to pick a random _splitting number T_ according to a density
> `f(t), f(t) > 0, for t ∈ (-∞, ∞)`. If the number in hand is less than _T_, decide that
> it is the smaller, if greater than _T_, decide that it is the larger.

This can be visualized by the following table, where the two randomly drawn numbers
are represented in order (first, second). For simplicity, we only compare against the
first random number as the probability distribution for both of them is equal.

| Numbers    | Random Draw | Test         | Outcome | Decision         | Correct |
|------------|-------------|--------------|---------|------------------|---------|
| **75**, 25 | 100         | 100 > **75** | true    | Second is higher | no      |
| **75**, 25 | 50          | 50 > **75**  | false   | Second is lower  | yes     |
| **75**, 25 | 0           | 0 > **25**   | false   | Second is lower  | yes     |
| **25**, 75 | 100         | 100 > **25** | true    | Second is higher | yes     |
| **25**, 75 | 50          | 50 > **25**  | true    | Second is higher | yes     |
| **25**, 75 | 0           | 0 > **25**   | false   | Second is lower  | no      |

According to this table, 4/6 (or 66%) of all guesses are correct.

## Simulation

This project implements a simulation on a two naive guessing strategies and the one described above.

```plain
Evaluating strategy: Always guess the same outcome (1000000 trials)
  Probability of correct guess: 50.05%
Evaluating strategy: Always guess a random outcome (1000000 trials)
  Probability of correct guess: 49.94%
Evaluating strategy: Comparison with a random draw (1000000 trials)
  Probability of correct guess: 66.69%
```

## Citation

```bibtex
@Inbook{Cover1987,
    author="Cover, Thomas M.",
    editor="Cover, Thomas M.
    and Gopinath, B.",
    title="Pick the Largest Number",
    bookTitle="Open Problems in Communication and Computation",
    year="1987",
    publisher="Springer New York",
    address="New York, NY",
    pages="152--152",
    abstract="Player 1 writes down any two distinct numbers on separate slips of paper. Player 2 randomly chooses one of these slips of paper and looks at the number. Player 2 must decide whether the number in his hand is the larger of the two numbers. He can be right with probability one-half. It seems absurd that he can do better.",
    isbn="978-1-4612-4808-8",
    doi="10.1007/978-1-4612-4808-8_43",
    url="https://doi.org/10.1007/978-1-4612-4808-8_43"
}
```
