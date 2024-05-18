use rand::distributions::Distribution;
use rand::Rng;
use rand_distr::Normal;

const LOW: i32 = i32::MIN;
const HIGH: i32 = i32::MAX;
const TRIALS: usize = 1000000;

type Strategy = fn(i32) -> Decision;

const STRATEGIES: [(&str, Strategy); 4] = [
    ("Always guess the same outcome", always_same_guess_strategy),
    ("Always guess a random outcome", random_guess_strategy),
    (
        "Comparison with a random draw (uniform distribution)",
        randomized_strategy,
    ),
    (
        "Comparison with a random draw (normal distribution)",
        randomized_normal_strategy,
    ),
];

fn main() {
    println!("Simulating strategies with {TRIALS} trials each.");
    for (name, strategy) in STRATEGIES {
        println!("Evaluating strategy: {name}");
        let success_rate = evaluate_strategy(strategy);
        println!(
            "  Probability of correct guess: {:.2}%",
            success_rate * 100.0
        )
    }
}

/// This strategy ignores the input and always returns the same outcome.
fn always_same_guess_strategy(_first: i32) -> Decision {
    Decision::SecondIsLower
}

/// This strategy ignores the input and always returns a random outcome.
///
/// This is essentially the same strategy as [`always_same_guess_strategy`] just with
/// a more randomized guessing scheme. Given that both numbers are drawn from a uniform
/// distribution, the chances of the new random being larger (or smaller) than the input
/// are 50% each, so no noteworthy change in outcome is expected.
fn random_guess_strategy(_first: i32) -> Decision {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        Decision::SecondIsHigher
    } else {
        Decision::SecondIsLower
    }
}

/// This strategy draws its own random number and compares it to the input. Based on this,
/// it will make a decision.
///
/// | First Number | Second Number | Random Draw | Test     | Outcome | Decision         | Correct |
/// |--------------|---------------|-------------|----------|---------|------------------|---------|
/// | 75           | 25            | 100         | 100 > 75 | true    | Second is higher | no      |
/// | 75           | 25            | 50          | 50 > 75  | false   | Second is lower  | yes     |
/// | 75           | 25            | 0           | 0 > 25   | false   | Second is lower  | yes     |
/// | 25           | 75            | 100         | 100 > 25 | true    | Second is higher | yes     |
/// | 25           | 75            | 50          | 50 > 25  | true    | Second is higher | yes     |
/// | 25           | 75            | 0           | 0 > 25   | false   | Second is lower  | no      |
///
/// According to this table, 4/6 (or 66%) of all guesses are correct.
fn randomized_strategy(first: i32) -> Decision {
    let comparison = rand_in_range(LOW, HIGH);
    if comparison >= first {
        Decision::SecondIsHigher
    } else {
        Decision::SecondIsLower
    }
}

/// Similar to [`randomized_strategy`], but using a normal distribution instead.
fn randomized_normal_strategy(first: i32) -> Decision {
    let comparison = rand_norm_in_range(0.0, 10.0);
    if comparison >= first {
        Decision::SecondIsHigher
    } else {
        Decision::SecondIsLower
    }
}

fn evaluate_strategy(strategy: Strategy) -> f32 {
    let mut correct = 0;
    for _ in 0..TRIALS {
        let first = rand_in_range(LOW, HIGH);
        let second = rand_in_range(LOW, HIGH);
        match evaluate_guess(first, second, &strategy) {
            Outcome::Incorrect => {}
            Outcome::Correct => correct += 1,
        }
    }

    (correct as f32) / (TRIALS as f32)
}

fn evaluate_guess(first: i32, second: i32, strategy: &Strategy) -> Outcome {
    let is_lower = second <= first;
    let is_higher = second >= first;
    let decision = strategy(first);
    match (decision, is_lower, is_higher) {
        (Decision::SecondIsLower, true, _) => Outcome::Correct,
        (Decision::SecondIsHigher, _, true) => Outcome::Correct,
        _ => Outcome::Incorrect,
    }
}

fn rand_in_range(low: i32, high: i32) -> i32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(low..=high)
}

fn rand_norm_in_range(mu: f64, sigma: f64) -> i32 {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(mu, sigma).expect("failed to get normal distribution");
    normal.sample(&mut rng) as i32
}

#[derive(Eq, PartialEq)]
enum Decision {
    SecondIsLower,
    SecondIsHigher,
}

#[derive(Eq, PartialEq)]
enum Outcome {
    Incorrect,
    Correct,
}
