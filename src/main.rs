
use std::env;

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("expression argument required");
    }

    //let expression = "0 * 1 +";
    let expression = args[1].clone();
    let mut char_iter = expression.chars();

    let mut nfas: Vec<NFA> = vec![];

    let mut current_node: usize = 0;
    let mut nfa_states: Vec<NFAState> = vec![];

    loop {
        if let Some(c) = char_iter.next() {

            match c {
                '+' => {
                    if nfas.len() < 2 {
                        panic!("malformed expression: not enough arguments to operator '{}' (expected 2)", c);
                    }

                    let n1 = nfas.pop().unwrap();
                    let n2 = nfas.pop().unwrap();

                    let t1 = Transition { accepting: 'ε', next: n1.start_state };
                    let t2 = Transition { accepting: 'ε', next: n2.start_state };

                    // ID is current_node
                    let start_state = NFAState { is_final: false, transitions: vec![t1, t2] };
                    // ID is current_node + 1
                    let final_state = NFAState { is_final: true, transitions: vec![] };

                    let mut previous_final_states: Vec<usize> = vec![];

                    for old_final in n1.final_states {
                        previous_final_states.push(old_final);
                    }

                    for old_final in n2.final_states {
                        previous_final_states.push(old_final);
                    }

                    for old_final in previous_final_states {
                        nfa_states[old_final].transitions.push(Transition { accepting: 'ε', next: current_node + 1 });
                        nfa_states[old_final].is_final = false;
                    }

                    nfa_states.push(start_state); 
                    nfa_states.push(final_state); 

                    nfas.push(NFA{ start_state: current_node, final_states: vec![current_node + 1] });

                    current_node = current_node + 2;
                
                },

                '.' => {
                    if nfas.len() < 2 {
                        panic!("malformed expression: not enough arguments to operator '{}' (expected 2)", c);
                    }

                    // Pop in reverse to concat in proper order
                    let n2 = nfas.pop().unwrap();
                    let n1 = nfas.pop().unwrap();

                    let mut previous_final_states: Vec<usize> = vec![];

                    for old_final in n1.final_states {
                        previous_final_states.push(old_final);
                    }

                    for old_final in previous_final_states {
                        nfa_states[old_final].transitions.push(Transition { accepting: 'ε', next: n2.start_state });
                        nfa_states[old_final].is_final = false;
                    }
                    
                    nfas.push(NFA{ start_state: n1.start_state, final_states: n2.final_states }); 

                },
               
                '*' => {
                    if nfas.len() < 1 {
                        panic!("malformed expression: not enough arguments to operator '{}' (expected 1)", c);
                    }

                    let n1 = nfas.pop().unwrap();

                    // Connection to previous start state
                    let t1 = Transition { accepting: 'ε', next: n1.start_state };
                    // Connection to new final node
                    let t2 = Transition { accepting: 'ε', next: current_node + 1 };

                    // ID is current_node
                    let start_state = NFAState { is_final: false, transitions: vec![t1, t2] };
                    // ID is current_node + 1
                    let final_state = NFAState { is_final: true, transitions: vec![] };

                    let mut previous_final_states: Vec<usize> = vec![];

                    for old_final in n1.final_states {
                        previous_final_states.push(old_final);
                    }

                    for old_final in previous_final_states {
                        // Make transitions to new end and start nodes
                        nfa_states[old_final].transitions.push(Transition { accepting: 'ε', next: current_node + 1 });
                        nfa_states[old_final].transitions.push(Transition { accepting: 'ε', next: current_node });

                        nfa_states[old_final].is_final = false;
                    }

                    nfa_states.push(start_state); 
                    nfa_states.push(final_state); 
                    
                    nfas.push(NFA{ start_state: current_node, final_states: vec![current_node + 1] }); 

                    current_node = current_node + 2;
                },

                '0' | '1' | 'a' | 'b' | 'c' => {
                    let transition = Transition { accepting: c, next: current_node + 1 }; 
                    let start_state = NFAState { is_final: false, transitions: vec![transition] };
                    let final_state = NFAState { is_final: true, transitions: vec![] };

                    nfa_states.push(start_state); 
                    nfa_states.push(final_state); 

                    nfas.push(NFA{ start_state: current_node, final_states: vec![current_node + 1] });

                    current_node = current_node + 2;

                },

                ' ' => continue,

                _ => panic!("invalid input char '{}'", c)
            }
        } else {
            break;
        }
    }

    println!("digraph finite_state_machine {{");
    println!("    rankdir=LR;");
    println!("    size=\"8,5\";");

    // Place the start node first so it appears on the far left
    println!("    node [shape = circle]; {} [ label=\"\"];", nfas[0].start_state);

    for (i, state) in nfa_states.iter().enumerate() {
        if state.is_final {
            println!("    node [shape = doublecircle]; {} [ label=\"\"];", i);
        } else {
            if i != nfas[0].start_state {
              println!("    node [shape = circle]; {} [ label=\"\"];", i);
            }
        }
    }

    println!("    node [shape = circle];");

    let mut i = 0;
    for state in nfa_states {
        for t in state.transitions {
            println!("    {} -> {} [ label =\"{}\" ];", i, t.next, t.accepting);
        }
        i = i + 1;
    }

    // Add an extra invisible node to the start node to get the starting arrow
    println!("    node [color = white]; {} [ label=\"\"]", -1);
    println!("    -1 -> {}", nfas[0].start_state);

    println!("}}");
}

// Example desired output
//digraph finite_state_machine {
//	rankdir=LR;
//	size="8,5"
//	node [shape = doublecircle]; LR_0 LR_3 LR_4 LR_8;
//	node [shape = circle];
//	LR_0 -> LR_2 [ label = "SS(B)" ];
//	LR_0 -> LR_1 [ label = "SS(S)" ];
//	LR_1 -> LR_3 [ label = "S($end)" ];
//	LR_2 -> LR_6 [ label = "SS(b)" ];
//	LR_2 -> LR_5 [ label = "SS(a)" ];
//	LR_2 -> LR_4 [ label = "S(A)" ];
//	LR_5 -> LR_7 [ label = "S(b)" ];
//	LR_5 -> LR_5 [ label = "S(a)" ];
//	LR_6 -> LR_6 [ label = "S(b)" ];
//	LR_6 -> LR_5 [ label = "S(a)" ];
//	LR_7 -> LR_8 [ label = "S(b)" ];
//	LR_7 -> LR_5 [ label = "S(a)" ];
//	LR_8 -> LR_6 [ label = "S(b)" ];
//	LR_8 -> LR_5 [ label = "S(a)" ];
//}

#[derive (Debug)]
struct NFA {
    start_state: usize,
    final_states: Vec<usize>
}

#[derive (Debug)]
struct NFAState {
    is_final: bool,
    transitions: Vec<Transition>,
}

#[derive (Debug)]
struct Transition {
    accepting: char,
    next: usize,
}
