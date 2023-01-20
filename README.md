# tv-utility-ai

## Usage
```rust
use tv_utility_ai::{WeightedAction, curve, choose};

fn make_work_action(money: f32) -> WeightedAction<&str> {
    WeightedAction::new(
        "work",
        curve::invert(curve::logistic(money, 4.0))
    )
}

fn make_sleep_action(tiredness: f32) -> WeightedAction<&str> {
    WeightedAction::new(
        "sleep",
        curve::quadratic(tiredness, 1.0, 2.0)
    )
}

fn make_eat_action(hunger: f32) -> WeightedAction<&str> {
    WeightedAction::new(
        "eat",
        curve::quadratic(hunger, 1.0, 3.0)
    )
}

let actions = vec![
    make_work_action(0.1),
    make_eat_action(0.5),
    make_sleep_action(0.3),
];

let best_action = choose::choose_action(actions);

```

## TODO
- finish tests
- write readme
- bench?