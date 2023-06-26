# tv_utility_ai

A simple utility-ai lib allowing people to create and choose actions using curves. This lib is based on this [chapter](http://www.gameaipro.com/GameAIPro/GameAIPro_Chapter09_An_Introduction_to_Utility_Theory.pdf) from Game AI Pro. 

## Usage
Generally speaking, using this means creating a series of `WeightedAction`s using curves to determine their weight, then passing them into one of the choosers, either `choose_action` or `choose_action_fuzzy`, which will return a single action.

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
// likely "work"

```

## TODO
- finish tests
- write readme
- bench?