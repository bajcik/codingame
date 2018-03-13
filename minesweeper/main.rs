/*   https://www.codingame.com/ide/puzzle/minesweeper
 *   Krzysztof Garus, 2018                            */
use std::io;
use std::vec::Vec;
use std::cmp;
use std::fmt;


/* ------------------------------------------------------------- CG macros */
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}


/* ------------------------------------------------------------- Data
    .-----------------+-------+---------------.
    |      .--(own)---| Board |---(own)--.    |
    |      |          `-------'          |    |
    |      V                             V    |
    | .--------.                    .-------. |
    | |        |--------[idx]------>|       | |
    | | Square |                    |  Hint | |
    | |        |<-------[idx]-------|       | |
    | `--------'                    `-------' |
    `-----------------------------------------'
*/

/* ------------------------------------------------------------- Struct*/

struct Board {
    w: u32,
    h: u32,
    bombs: u32,
    squares: Vec<Square>,
    hints: Vec<Hint>,
    stack: Vec<usize>,  // squares to investigate
}

#[derive(PartialEq)]
struct Hint {
    bombs: u32,
    squares: Vec<usize>,  // sorted
    valid: bool,
}

#[derive(Debug)]
struct Square {
    state: SquareState,
    on_stack: bool,
    hints: Vec<usize>,
}

#[derive(PartialEq,Debug)]
enum SquareState {
    Unknown,
    Empty,
    Bomb,
}

#[derive(PartialEq)]
struct Delta {
    hints2rm: Vec<usize>,
    hints2add: Vec<Hint>,
    squares2bomb: Vec<usize>,
    squares2empty: Vec<usize>,
}

/* ------------------------------------------------------------- Impl */
impl Hint {
    fn new(bombs: u32, mut squares: Vec<usize>) -> Hint {
        squares.sort();
        squares.dedup();
        assert!(bombs <= squares.len() as u32);
        let empty = squares.len() == 0;
        Hint { bombs: bombs, squares: squares, valid: !empty }
    }

    // return (h1, h2, both) square number sets
    // Ex: compare_squares([1,3,12,13,14],[7,8,9,12,13,22]) -> ([1,3,14], [7,8,9,22], [12,13])
    fn compare_squares(hint1: &Hint, hint2: &Hint) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
        let mut squares_both : Vec<usize> = Vec::new(); // [1,4]
        let mut squares_h1 : Vec<usize> = Vec::new();   // [2]
        let mut squares_h2 : Vec<usize> = Vec::new();   // [3,8]

        let mut it1 = hint1.squares.iter();
        let mut it2 = hint2.squares.iter();
        let mut next1 = it1.next();
        let mut next2 = it2.next();
        loop {
            match (next1, next2) {
                (Some(c1), Some(c2)) => {
                    if c1 == c2 { squares_both.push(*c1); next1 = it1.next(); next2 = it2.next() }
                    else if c1 < c2 { squares_h1.push(*c1); next1 = it1.next() }
                    else { squares_h2.push(*c2); next2 = it2.next() }
                }
                (Some(c1), None) => { squares_h1.push(*c1); next1 = it1.next() },
                (None, Some(c2)) => { squares_h2.push(*c2); next2 = it2.next() },
                (None, None) => break,
            }
        }

        (squares_h1, squares_h2, squares_both)
    }
}

impl Square {
    fn new() -> Square {
        Square { state: SquareState::Unknown, hints: Vec::with_capacity(8), on_stack: false, }
    }

    fn add_hint(&mut self, h: usize) {
        self.hints.push(h);
    }

    fn rm_hint(&mut self, h: usize) {
        let idx = self.hints.iter().position(|x| *x == h).unwrap();
        self.hints.swap_remove(idx);
    }
}

impl Delta {
    fn new() -> Delta {
        Delta { hints2rm: Vec::new(), hints2add: Vec::new(), squares2bomb: Vec::new(), squares2empty: Vec::new(), }
    }

    fn add_hint(&mut self, hint: Hint) {
        if ! hint.valid { return; }  
        self.hints2add.push(hint);
    }
    fn rm_hint(&mut self, h: usize) {
        self.hints2rm.push(h);
    }
    fn set_square_bomb(&mut self, s: usize) {
        self.squares2bomb.push(s);
    }
    fn set_square_empty(&mut self, s: usize) {
        self.squares2empty.push(s);
    }

    fn reset(&mut self) {
        self.hints2rm.truncate(0);
        self.hints2add.truncate(0);
        self.squares2bomb.truncate(0);
        self.squares2empty.truncate(0);
    }
    fn is_empty(&self) -> bool {
        self.hints2rm.is_empty() &&
        self.hints2add.is_empty() &&
        self.squares2bomb.is_empty() &&
        self.squares2empty.is_empty()
    }
}

/* ------------------------------------------------------------- Impl: Board I/O */
impl Board {
    // make board with Empty Square's
    fn new(w: u32, h: u32, bombs: u32) -> Board {
        let mut squares: Vec<Square> = Vec::with_capacity((w*h) as usize);
        for _ in 0..w*h {
            squares.push( Square::new() );
        }
        Board { w: w,
                h: h,
                bombs: bombs,
                squares: squares,
                hints: Vec::new(),
                stack: Vec::new(),
        }
    }

    fn from_stdin() -> Board {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let h = parse_input!(inputs[0], u32);
        let w = parse_input!(inputs[1], u32);
        
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let nb = parse_input!(input_line, u32);
        
        let mut board = Board::new(w, h, nb);
        
        for y in 0..h {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let line = input_line.trim_right().to_string();
            
            for (ix,ch) in line.chars().enumerate() {
                let x = ix as u32;
                let s = board.xy2s(x, y);
                match ch {
                    '.' => { let s = board.xy2s(x,y); board.set_square_state(s, SquareState::Empty); },
                    '?' => {}
                    '1' => { let hint = board.bomb2hint(x, y, 1); board.add_hint(hint); board.set_square_state(s, SquareState::Empty); },
                    '2' => { let hint = board.bomb2hint(x, y, 2); board.add_hint(hint); board.set_square_state(s, SquareState::Empty); },
                    '3' => { let hint = board.bomb2hint(x, y, 3); board.add_hint(hint); board.set_square_state(s, SquareState::Empty); },
                    '4' => { let hint = board.bomb2hint(x, y, 4); board.add_hint(hint); board.set_square_state(s, SquareState::Empty); },
                    '5' => { let hint = board.bomb2hint(x, y, 5); board.add_hint(hint); board.set_square_state(s, SquareState::Empty); },
                    '6' => { let hint = board.bomb2hint(x, y, 6); board.add_hint(hint); board.set_square_state(s, SquareState::Empty); },
                    '7' => { let hint = board.bomb2hint(x, y, 7); board.add_hint(hint); board.set_square_state(s, SquareState::Empty); },
                    '8' => { let hint = board.bomb2hint(x, y, 8); board.add_hint(hint); board.set_square_state(s, SquareState::Empty); },
                    _ => unreachable!(),
                }
            }
        }

        board
    }

    fn print_bombs(&self) {
        let mut found_bombs = 0;
        for x in 0..self.w {
            for y in 0..self.h {
                if self.squares[self.xy2s(x,y)].state == SquareState::Bomb {
                    println!("{} {}", x, y);
                    found_bombs += 1;
                }
            }
        }
        eprintln!("Found {}/{} bombs", found_bombs, self.bombs);
    }

    // converstions Square ID <--> [X,Y]
    // Used: - when reading a grid and making Hints
    //       - when printing answer
    fn xy2s(&self, x: u32, y: u32) -> usize {
        assert!(x < self.w && y < self.h);
        (self.w*y + x ) as usize
    }
    
    fn bomb2hint(&self, x: u32, y: u32, bombs: u32) -> Hint {
        let mut squares :Vec<usize> = Vec::with_capacity(8);

        if x > 0        && y > 0        { squares.push(self.xy2s(x-1,y-1)); }
        if                 y > 0        { squares.push(self.xy2s(x  ,y-1)); }
        if x < self.w-1 && y > 0        { squares.push(self.xy2s(x+1,y-1)); }
        if x < self.w-1                 { squares.push(self.xy2s(x+1,y  )); }
        if x < self.w-1 && y < self.h-1 { squares.push(self.xy2s(x+1,y+1)); }
        if                 y < self.h-1 { squares.push(self.xy2s(x  ,y+1)); }
        if x > 0        && y < self.h-1 { squares.push(self.xy2s(x-1,y+1)); }
        if x > 0                        { squares.push(self.xy2s(x-1,y  )); }

        Hint::new(bombs, squares)
    }


/* ------------------------------------------------------------- Impl: Board update */
    // updates squares & stack
    fn add_hint(&mut self, hint: Hint) {
        let h = self.hints.len();
        for s in hint.squares.iter() {
            self.update_square(*s);
            self.squares[*s].add_hint(h);
        }
        self.hints.push(hint);
    }

    fn rm_hint(&mut self, h: usize) {
        assert!(h < self.hints.len());
        for s in self.hints[h].squares.iter() {
            self.squares[*s].rm_hint(h);
        }
        
        self.hints[h].valid = false;

        // tidy vector, not chaning position of any valid hint
        while self.hints.len() > 0 {
            if ! self.hints.last().unwrap().valid {
                let last_idx = self.hints.len()-1;
                self.hints.remove(last_idx);
            } else {
                break;
            }
        }
    }

    fn update_square(&mut self, s: usize) {
        if ! self.squares[s].on_stack {
            self.stack.push(s);
            self.squares[s].on_stack = true;
        }
    }

    fn set_square_state(&mut self, s: usize, state: SquareState) {
        if self.squares[s].state != state {
            assert!(self.squares[s].state == SquareState::Unknown);
            self.squares[s].state = state;
            self.update_square(s);
        }
    }

    fn update_delta(&mut self, delta: &mut Delta) {
        eprintln!("{}", delta);
        for h in delta.hints2rm.iter() { self.rm_hint(*h); }
        while let Some(hint) = delta.hints2add.pop() { self.add_hint(hint); }
        for s in delta.squares2bomb.iter() { self.set_square_state(*s, SquareState::Bomb); }
        for s in delta.squares2empty.iter() { self.set_square_state(*s, SquareState::Empty); }
        delta.reset();
        eprintln!("{}", self);
    }
    
/* ------------------------------------------------------------- Impl: Board: Hint processing */
    // may put other hints
    // returns TRUE if hint should be released
    fn process_hint(&self, h: usize, delta: &mut Delta) {
        let hint = & self.hints[h];
        eprintln!("process_hint({}={})", h, hint);

        // "some squares already defined"
        // Ex: 4[??*.] -> 2[??]
        if ! hint.squares.iter().all(|&s| self.squares[s].state == SquareState::Unknown) {
            let mut squares2 : Vec<usize>= Vec::with_capacity(hint.bombs as usize);
            let mut bombs2 = hint.bombs;
            let mut squares_it = hint.squares.iter();
            while let Some(s) = squares_it.next() {
                match self.squares[*s].state {
                    SquareState::Unknown => squares2.push(*s),
                    SquareState::Empty => {},
                    SquareState::Bomb => bombs2 -= 1,
                }
            }
            delta.add_hint(Hint::new(bombs2, squares2));
            delta.rm_hint(h);
            return;
        }

        // "no bombs"
        // Ex: 0[.?*?] -> 00*0
        if hint.bombs == 0 {
            for s in &hint.squares {
                delta.set_square_empty(*s);
            }
            delta.rm_hint(h);
            return;
        }

        // "all bombs"
        // Ex: 2[??] -> **
        if hint.bombs  == hint.squares.len() as u32 {
            for s in &hint.squares {
                delta.set_square_bomb(*s);
            }
            delta.rm_hint(h);
            return;
        }
    }

    fn process_hint_pair(&self, h1: usize, h2: usize, delta: &mut Delta) {
        let hint1 = & self.hints[h1];
        let hint2 = & self.hints[h2];
        
        // handy assumption for "h1 inside h2" case
        if hint1.squares.len() > hint2.squares.len() {
            self.process_hint_pair(h2, h1, delta);
            return;
        }
        eprintln!("process_hint_pair({}={}, {}={})", h1, hint1, h2, hint2);
                                         // Ex: h1=[1,2,4] h2=[1,3,4,8]
        
        let (squares_h1, squares_h2, squares_both) = Hint::compare_squares(hint1, hint2);
        assert!(squares_both.len() > 0); // hints must be related to the same square
    
        // duplicated -> delete one
        // Ex: 2[11,12,13] -> 2[11,12,13]
        //     2[11,12,13]
        if squares_h1.len() == 0 && squares_h2.len() == 0 {
            delta.rm_hint(h1);
            return;
        }

        // h1 inside h2
        // Ex: 1[10,11]       -> 1[10,11]
        //     3[10,11,12,13] ->        2[12,13]
        if squares_h1.len() == 0 {
            assert!(hint1.bombs <= hint2.bombs);
            assert!(squares_h2.len() > 0);
           
            delta.add_hint( Hint::new(hint2.bombs-hint1.bombs, squares_h2) );
            delta.rm_hint(h2);
            return;
        }

        // intersecing square vectors
        // (try to) calculate number of bombs in squares_both section
        // 
        //     <---X---><---Y---><---Z--->   <- nr of squares in segment
        // h1: [   A   ][   B   ]            <- h1 hint with M bombs (M=A+B)
        // h2:          [   B   ][   C   ]   <- h2 hint with N bombs (N=B+C)
        //
        // So: B >= M-X, B >= N-Z            <- if can't fit all M bombs in X, or N bombs in Z
        //     B <= Y, B >= 0                <- of course
        //     B <= M, B <= N
        //
        // Ex: 1[10,11,12,13,14]     -> 0[10,11,12]      1[15]
        //              2[13,14,15]  ->           1[13,14]
        let mut both_min = 0u32;
        if hint1.bombs >= squares_h1.len() as u32 {
            both_min = cmp::max(both_min, hint1.bombs - squares_h1.len() as u32);
        }
        if hint2.bombs >= squares_h2.len() as u32 {
            both_min = cmp::max(both_min, hint2.bombs - squares_h2.len() as u32);
        }
        let mut both_max = squares_both.len() as u32;
        both_max = cmp::min(both_max, hint1.bombs );
        both_max = cmp::min(both_max, hint2.bombs );
        if both_min == both_max {
            delta.add_hint( Hint::new(both_min, squares_both) );
            delta.add_hint( Hint::new(hint1.bombs - both_min, squares_h1) );
            delta.add_hint( Hint::new(hint2.bombs - both_min, squares_h2) );
            delta.rm_hint(h1);
            delta.rm_hint(h2);
            return;
        }
    }

    fn process_remaining(&self, delta: &mut Delta) {
        let mut n_bombs = 0;
        let mut n_unknown = 0;
        for square in self.squares.iter() {
            match square.state {
                SquareState::Bomb => n_bombs += 1,
                SquareState::Unknown => n_unknown += 1,
                _ => {},
            }
        }

        if n_bombs < self.bombs && n_bombs+n_unknown == self.bombs {
            for s in 0..(self.w*self.h) as usize {
                if self.squares[s].state == SquareState::Unknown {
                    delta.set_square_bomb(s);
                }
            }
        }
    }

    fn process_stack(&mut self) {
        let mut delta = Delta::new();
        eprintln!("process_stack begin, board={}", self);


        'next_square: while let Some(s) = self.stack.pop() {
            eprintln!("square {}={:?}", s, self.squares[s]);
            self.squares[s].on_stack = false;

            // process each Hint
            for h in &self.squares[s].hints {
                self.process_hint(*h, &mut delta);
                if ! delta.is_empty() { break; } // for easier debugging
            }

            // do delta
            if ! delta.is_empty() {
                self.update_delta(&mut delta);
                self.update_square(s);
                continue 'next_square;
            }

            // process Hint pairs
            'pairs: for (i, h1) in self.squares[s].hints.iter().enumerate() {
                for (_, h2) in self.squares[s].hints.iter().enumerate().filter(|&(j,_)| j>i) {
                    self.process_hint_pair(*h1, *h2, &mut delta);
                    if ! delta.is_empty() { break 'pairs; } // for easier debugging
                }
            }
           
            // do delta
            if ! delta.is_empty() {
                self.update_delta(&mut delta);
                self.update_square(s);
            }
        }

        // x uncovered bombs on x unknown squares
        self.process_remaining(&mut delta);
        self.update_delta(&mut delta);
        
        eprintln!("process_stack end, board={}", self);
    }

}

/* ------------------------------------------------------------- Impl: Display for ... */
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n.--- Square bombs={} {}x{}", self.bombs, self.w, self.h)?;
        
        // squares
        for y in 0..self.h {
            write!(f, "| {:3}> ", self.xy2s(0,y))?;
            for x in 0..self.w {
                write!(f, "{}", self.squares[self.xy2s(x,y)])?;
            }
            writeln!(f, "")?;
        }

        // hints
        write!(f, "| Hints: ")?;
        for (i,hint) in self.hints.iter().enumerate().filter(|&(_,h)| h.valid) {
            write!(f, "{}:{} ", i, hint)?;
        }
        writeln!(f, "")?;

        // stack
        write!(f, "| Stack: ")?;
        for s in self.stack.iter() {
            write!(f, "{}, ", s)?;
        }
        writeln!(f, "\n`---\n")
    }
}

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n.--- Delta")?;
        
        for hint in self.hints2add.iter() {
            writeln!(f, "| +Hint: {}", hint)?;
        }

        for h in self.hints2rm.iter() {
            writeln!(f, "| -Hint: {}", h)?;
        }

        for s in self.squares2bomb.iter() {
            writeln!(f, "| Bomb: {}", s)?;
        }

        for s in self.squares2empty.iter() {
            writeln!(f, "| Empty: {}", s)?;
        }

        writeln!(f, "`---")
    }
}
impl fmt::Debug for Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl fmt::Display for Hint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        if self.valid {
            write!(f, "{}@", self.bombs)?;
            for s in &self.squares {
                write!(f, "{},", s)?;
            }
        } else {
            write!(f, "INVALID")?;
        }
        write!(f, ")")
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner = match self.state {
            SquareState::Bomb => format!("*"),
            SquareState::Empty => format!("."),
            SquareState::Unknown if self.hints.len() == 0 => format!("?"),
            SquareState::Unknown  => format!("{}", self.hints.len()),
        };

        if self.on_stack {
            write!(f, "[{}]", inner)?;
        } else {
            write!(f, " {} ", inner)?;
        }
        Ok(())
    }
}

/* ------------------------------------------------------------- main */
#[cfg(not(test))]
fn main() {
    let mut board = Board::from_stdin();
    board.process_stack();
    board.print_bombs();
}


/* ------------------------------------------------------------- tests */
#[test]
fn process_h_0len() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:3
    ], hints: vec![
        Hint::new(1, vec![0,2]),                                                 // h:0
        Hint::new(0, vec![]),                                                    // h:1
    ]};

    let mut delta = Delta::new();
    board.process_hint(1, &mut delta);
    assert_eq!(delta, Delta {
        hints2rm: vec![1],
        hints2add: vec![],
        squares2bomb: vec![],
        squares2empty: vec![],
    });
}

#[test]
fn process_h_0delta() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:3
    ], hints: vec![
        Hint::new(1, vec![0,2]),                                                 // h:0
    ]};

    let mut delta = Delta::new();
    board.process_hint(0, &mut delta);
    assert_eq!(delta, Delta {
        hints2rm: vec![],
        hints2add: vec![],
        squares2bomb: vec![],
        squares2empty: vec![],
    });
}

#[test]
fn process_h_known_bomb() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:3
    ], hints: vec![
        Hint::new(2, vec![0,1,3]),                                               // h:0
    ]};

    let mut delta = Delta::new();
    board.process_hint(0, &mut delta);

    assert_eq!(delta, Delta {
        hints2rm: vec![0],
        hints2add: vec![ Hint::new(0, vec![0]) ],
        squares2bomb: vec![],
        squares2empty: vec![],
    });
}

#[test]
fn process_h_known_empty() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:3
    ], hints: vec![
        Hint::new(2, vec![1,2,3]),                                               // h:0
    ]};

    let mut delta = Delta::new();
    board.process_hint(0, &mut delta);

    assert_eq!(delta, Delta {
        hints2rm: vec![0],
        hints2add: vec![ Hint::new(2, vec![1,3]) ],
        squares2bomb: vec![],
        squares2empty: vec![],
    });
}

#[test]
fn process_h_known_bomb_empty() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:3
    ], hints: vec![
        Hint::new(2, vec![0,1,2,3]),                                             // h:0
    ]};

    let mut delta = Delta::new();
    board.process_hint(0, &mut delta);

    assert_eq!(delta, Delta {
        hints2rm: vec![0],
        hints2add: vec![ Hint::new(1, vec![0,3]) ],
        squares2bomb: vec![],
        squares2empty: vec![],
    });
}

#[test]
fn process_h_known_all() {
    let board = Board { h:0, w:4, bombs:2, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:3
    ], hints: vec![
        Hint::new(2, vec![0,1,2,3]),                                             // h:0
    ]};

    let mut delta = Delta::new();
    board.process_hint(0, &mut delta);

    assert_eq!(delta, Delta {
        hints2rm: vec![0],
        hints2add: vec![],  // empty vector becames invalid, so it was not added to delta
        squares2bomb: vec![],
        squares2empty: vec![],
    });
}

#[test]
fn process_h_all_bombs() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Bomb, },    // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:3
    ], hints: vec![
        Hint::new(2, vec![0,3]),                                                 // h:0
    ]};

    let mut delta = Delta::new();
    board.process_hint(0, &mut delta);

    assert_eq!(delta, Delta {
        hints2rm: vec![0],
        hints2add: vec![],
        squares2bomb: vec![0,3],
        squares2empty: vec![],
    });
}

#[test]
fn process_h_all_empty() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:3
    ], hints: vec![
        Hint::new(0, vec![0,3]),                                                 // h:0
    ]};

    let mut delta = Delta::new();
    board.process_hint(0, &mut delta);

    assert_eq!(delta, Delta {
        hints2rm: vec![0],
        hints2add: vec![],
        squares2bomb: vec![],
        squares2empty: vec![0,3],
    });
}

#[test]
fn process_p_inside() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:3
    ], hints: vec![
        Hint::new(3, vec![0,1,2,3]),                                             // h:0
        Hint::new(1, vec![0,2]),                                                 // h:1
    ]};

    let mut delta = Delta::new();
    board.process_hint_pair(0, 1, &mut delta);

    assert_eq!(delta, Delta {
        hints2rm: vec![0],
        hints2add: vec![ Hint::new(2, vec![1,3]) ],
        squares2bomb: vec![],
        squares2empty: vec![],
    });
}

#[test]
fn process_p_intersect() {
    let board = Board { h:0, w:4, bombs:1, stack: vec![], squares: vec![
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:0
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:1
        Square { on_stack: false, hints: vec![], state: SquareState::Empty, },   // s:2
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:3
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:4
        Square { on_stack: false, hints: vec![], state: SquareState::Unknown, }, // s:5
    ], hints: vec![
        Hint::new(1, vec![0,1,2,3,4]),                                           // h:0
        Hint::new(2, vec![3,4,5]),                                               // h:1
    ]};

    let mut delta = Delta::new();
    board.process_hint_pair(0, 1, &mut delta);

    assert_eq!(delta, Delta {
        hints2rm: vec![1,0], // 1st shorter than 0th, so function was recursive called
        hints2add: vec![ Hint::new(1, vec![3,4]), Hint::new(1, vec![5]), Hint::new(0, vec![0,1,2]),], 
        squares2bomb: vec![],
        squares2empty: vec![],
    });
}

#[test]
fn hint_compare() {
    let hint1 = Hint::new(2,vec![1,3,12,13,14]);
    let hint2 = Hint::new(3,vec![7,8,9,12,13,22]);
    
    let (sq1, sq2, sqboth) = Hint::compare_squares(&hint1, &hint2);
    assert_eq!((sq1, sq2, sqboth), (vec![1,3,14], vec![7,8,9,22], vec![12,13]));
}


