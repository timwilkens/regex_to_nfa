cargo build  && ./target/debug/regex_to_nfa "$1" > nfa.gv && dot -Tpng nfa.gv -o nfa.png && open nfa.png
