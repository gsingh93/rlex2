use parse_regex::RegExParser;
use parse_regex::RegEx::{self, Or, Repetition, Sequence, Terminal};
use automata::NFA;
use automata::Transition::{Input, Epsilon};

use std::collections::HashMap;

macro_rules! set {
    ($($elem:expr),*) => ({
        let mut s = ::std::collections::HashSet::new();
        $(s.insert($elem);)*
        s
    })
}

macro_rules! map {
    ($($key:expr => $val:expr),*) => ({
        let mut h = ::std::collections::HashMap::new();
        $(h.insert($key, $val);)*
        h
    })
}

type State = usize;

pub struct Lexer<T> {
    state_id: State,
    nfas: Vec<NFA<State>>,
    tok_map: HashMap<State, T>
}

impl<T> Lexer<T> {
    fn new() -> Lexer<T> {
        Lexer { state_id: 0, nfas: Vec::new(), tok_map: HashMap::new() }
    }

    pub fn add_token(&mut self, regex: &str, token: T) {
        let mut p = RegExParser::new(regex.to_string());
        let nfa = match p.parse() {
            Ok(r) => self.regex_to_nfa(r),
            Err(e) => panic!("Error in regex: {}", e)
        };
        let accept_states: Vec<_> = nfa.get_accept_states().clone().into_iter().collect();
        self.nfas.push(nfa);
        assert!(accept_states.len() == 1);
        self.tok_map.insert(accept_states[0], token);
    }

    pub fn lex(&self, _: &str) -> Vec<T> {
        // let nfa = merge_nfas(self.nfas);
        // let dfa = nfa.into_dfa();
        // Implement DFA iterator, iterate until no more transitions, take last accept state
        // Get token from tok_map and add to toks
        let toks = Vec::new();
        toks
    }

    fn regex_to_nfa(&mut self, r: RegEx) -> NFA<State> {
        match r {
            Or(box r1, box r2) => self.construct_or_nfa(r1, r2),
            Sequence(v) => self.construct_sequence_nfa(v),
            Repetition(box r) => self.construct_repetition_nfa(r),
            Terminal(c) => self.construct_terminal_nfa(c)
        }
    }

    fn construct_or_nfa(&mut self, r1: RegEx, r2: RegEx) -> NFA<State> {
        let start = self.get_id();
        let n1 = self.regex_to_nfa(r1);
        let n2 = self.regex_to_nfa(r2);
        let end = self.get_id();

        let mut m1 = n1.get_transitions().clone();
        let m2 = n2.get_transitions().clone();
        for (trans, state) in m2.into_iter() {
            match m1.entry(trans).get() {
                Ok(_) => panic!("Duplicate states"),
                Err(e) => { e.insert(state); }
            }
        }
        let accept1: Vec<_> = n1.get_accept_states().clone().into_iter().collect();
        let accept2: Vec<_>  = n2.get_accept_states().clone().into_iter().collect();
        assert_eq!(accept1.len(), 1);
        assert_eq!(accept2.len(), 1);
        m1.insert((start, Epsilon), set!(*n1.get_start_state(), *n2.get_start_state()));
        m1.insert((accept1[0], Epsilon), set!(end));
        m1.insert((accept2[0], Epsilon), set!(end));

        NFA::new(start, set!(end), m1)
    }

    fn construct_sequence_nfa(&mut self, v: Vec<Box<RegEx>>) -> NFA<State> {
        let init_start = self.get_id();
        let mut end = init_start;
        let mut m = HashMap::new();
        for r in v {
            let start = end;
            end = self.get_id();

            let n = self.regex_to_nfa(*r);
            let map = n.get_transitions().clone();
            for (trans, state) in map.into_iter() {
                match m.entry(trans).get() {
                    Ok(_) => panic!("Duplicate states"),
                    Err(e) => { e.insert(state); }
                }
            }
            let accept: Vec<_> = n.get_accept_states().clone().into_iter().collect();
            assert_eq!(accept.len(), 1);
            m.insert((start, Epsilon), set!(*n.get_start_state()));
            m.insert((accept[0], Epsilon), set!(end));
        }

        NFA::new(init_start, set!(end), m)
    }

    fn construct_repetition_nfa(&mut self, r: RegEx) -> NFA<State> {
        let start = self.get_id();
        let n = self.regex_to_nfa(r);
        let end = self.get_id();

        let mut m = n.get_transitions().clone();
        let accept: Vec<_> = n.get_accept_states().clone().into_iter().collect();
        assert_eq!(accept.len(), 1);
        m.insert((start, Epsilon), set!(*n.get_start_state(), end));
        m.insert((accept[0], Epsilon), set!(*n.get_start_state(), end));

        NFA::new(start, set!(end), m)
    }

    fn construct_terminal_nfa(&mut self, c: char) -> NFA<State> {
        let start = self.get_id();
        let end = self.get_id();
        NFA::new(start, set!(end), map!((start, Input(c)) => set!(end)))
    }

    fn get_id(&mut self) -> State {
        let id = self.state_id;
        self.state_id += 1;
        id
    }
}

#[cfg(test)]
mod test {
    use lexer::Lexer;

    #[derive(Debug)]
    enum Token {
        IF,
        WHILE,
        NUM
    }

    #[test]
    fn test_main() {
        let mut lexer = Lexer::new();
        lexer.add_token("if", Token::IF);
        lexer.add_token("while", Token::WHILE);
        lexer.add_token("(0|1)|2", Token::NUM);

        for token in lexer.lex("IF WHILE 0 1 2").iter() {
            println!("{:?}", token);
        }
    }
}
