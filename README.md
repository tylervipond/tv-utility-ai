# tv-utility-ai

## TODO
- finish tests
- write readme
- bench?

## Usage
```rust
use tv_utility_ai::WeightedAction

fn make_work_action() -> WeightedAction<&str> {
    WeightedAction::new(
        "work"
        curve
    )
}

fn make_sleep_action() -> WeightedAction<&str> {
    WeightedAction::new(
        "sleep"
        curve
    )
}

fn make_eat_action(hunger: i32) -> WeightedAction<&str> {
    WeightedAction::new(
        "eat"
        curve::quadratic()
    )
}

tv_utility_ai::choose_action(vec![]);

```