use super::WeightedAction;
use std::collections::BinaryHeap;

/**
 * This returns whichever action has the highest weight
 */
pub fn choose_action<T: PartialEq>(weighted_actions: Vec<WeightedAction<T>>) -> Option<T> {
    weighted_actions
        .into_iter()
        .max()
        .map(|weighted_action| weighted_action.action)
}

/**
 * This returns one of the hightest weighted actions based on fuzziness and a passed in
 * choice_offset. The choice_offset should likely be populated with a random number by
 * the consumer.
 */
pub fn choose_action_fuzzy<T: PartialEq>(
    weighted_actions: Vec<WeightedAction<T>>,
    // a number between 0.0 and 1.0
    fuzzyness: f32,
    // a number between 0.0 and 1.0
    choice_offset: f32,
) -> Option<T> {
    let mut heap = BinaryHeap::from(weighted_actions);
    let greatest = heap.pop();
    if let Some(greatest) = greatest {
        let greatest_weight = greatest.weight;
        let mut total_weight = greatest_weight;
        let mut options = vec![greatest];
        while let Some(option) = heap.pop() {
            if greatest_weight - option.weight > fuzzyness {
                break;
            }
            total_weight += option.weight;
            options.push(option)
        }
        let mut option_offset = 0.0;
        for option in options {
            option_offset += option.weight / total_weight;
            if choice_offset <= option_offset {
                return Some(option.action);
            }
        }
    }
    None
}

#[cfg(test)]
mod test_choose_action {
    use super::*;

    #[test]
    fn returns_the_largest_action_from_a_list_of_actions() {
        let expected = "expected_action";
        let options = vec![
            WeightedAction {
                action: "unexpected_action_one",
                weight: 0.1,
            },
            WeightedAction {
                action: "unexpected_action_two",
                weight: 0.2,
            },
            WeightedAction {
                action: expected,
                weight: 0.5,
            },
            WeightedAction {
                action: "unexpected_action_four",
                weight: 0.4,
            },
        ];
        assert_eq!(choose_action(options), Some(expected));
    }

    #[test]
    fn returns_the_last_action_from_a_list_where_the_weights_are_equal() {
        let expected = "expected_action";
        let options = vec![
            WeightedAction {
                action: "unexpected_action_one",
                weight: 0.1,
            },
            WeightedAction {
                action: "unexpected_action_two",
                weight: 0.1,
            },
            WeightedAction {
                action: "unexpected_action_three",
                weight: 0.1,
            },
            WeightedAction {
                action: expected,
                weight: 0.1,
            },
        ];
        assert_eq!(choose_action(options), Some(expected));
    }

    #[test]
    fn returns_none_if_there_are_no_options() {
        let options: Vec<WeightedAction<&str>> = vec![];
        assert_eq!(choose_action(options), None);
    }
}

#[cfg(test)]
mod test_choose_action_fuzzy {
    use super::*;

    #[test]
    fn returns_the_largest_action_from_a_list_of_actions() {
        let expected = "expected_action";
        let options = vec![
            WeightedAction {
                action: "unexpected_action_one",
                weight: 0.1,
            },
            WeightedAction {
                action: "unexpected_action_two",
                weight: 0.2,
            },
            WeightedAction {
                action: expected,
                weight: 0.5,
            },
            WeightedAction {
                action: "unexpected_action_four",
                weight: 0.4,
            },
        ];
        assert_eq!(choose_action_fuzzy(options.clone(), 0.0, 0.0), Some(expected));
        assert_eq!(choose_action_fuzzy(options, 0.0, 1.0), Some(expected));
    }

    #[test]
    fn returns_an_action_based_on_weights_fuzziness_and_choice_offset() {
        let expected = "expected_action";
        let expected_2 = "expected_action_two";
        let options = vec![
            WeightedAction {
                action: "unexpected_action_one",
                weight: 0.1,
            },
            WeightedAction {
                action: expected_2,
                weight: 1.0,
            },
            WeightedAction {
                action: "unexpected_action_three",
                weight: 0.1,
            },
            WeightedAction {
                action: expected,
                weight: 0.95,
            },
        ];
        assert_eq!(choose_action_fuzzy(options.clone(), 0.1, 0.8), Some(expected));
        assert_eq!(choose_action_fuzzy(options, 0.1, 0.4), Some(expected_2));
    }

    #[test]
    fn returns_none_if_there_are_no_options() {
        let options: Vec<WeightedAction<&str>> = vec![];
        assert_eq!(choose_action(options), None);
    }
}
