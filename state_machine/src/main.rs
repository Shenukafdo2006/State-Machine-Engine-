use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum State {
    Idle,
    Processing,
    Completed,
    Error,
}

struct StateMachine {
    current_state: State,
    transitions: HashMap<State, Vec<State>>,
}

impl StateMachine {
    fn new() -> Self {
        let mut transitions = HashMap::new();

        transitions.insert(State::Idle, vec![State::Processing]);
        transitions.insert(State::Processing, vec![State::Completed, State::Error]);
        transitions.insert(State::Error, vec![State::Idle]);
        transitions.insert(State::Completed, vec![]);

        StateMachine {
            current_state: State::Idle,
            transitions,
        }
    }

    fn transition(&mut self, next: State) {
        if let Some(valid_states) = self.transitions.get(&self.current_state) {
            if valid_states.contains(&next) {
                println!("Transition: {:?} → {:?}", self.current_state, next);
                self.current_state = next;
            } else {
                println!("Invalid transition from {:?} to {:?}", self.current_state, next);
            }
        }
    }
}

fn main() {
    let mut sm = StateMachine::new();

    sm.transition(State::Processing);
    sm.transition(State::Completed);
    sm.transition(State::Idle); // invalid transition
}