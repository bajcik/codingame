use std::io;
use std::fmt;
use std::vec::Vec;
use self::PieceKind::*;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = input_line.trim().parse::<u32>().unwrap();

    let mut board = Board::from_stdin();
    eprintln!("{}", board);

    board.group();
    eprintln!("{}", board);
    
    let edg_vec = board.analysis();
    eprintln!("Groups: {:?}", edg_vec);

    println!("{}", answer(edg_vec, n));
}

struct Spot {
    x: usize,
    y: usize,
}

#[ derive(PartialEq, Copy, Clone) ]
enum PieceKind {
    Empty,      // empty space
    Piece,      // paper, not groupped yet
    Group(u16), // paper, groupped
}

#[ derive(Default,Clone) ]
struct GroupEdges {
    top : bool,
    bottom : bool,
    left : bool,
    right : bool,
}
impl fmt::Debug for GroupEdges
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}",
               if self.left   { "L" } else { "." },
               if self.right  { "R" } else { "." },
               if self.top    { "T" } else { "." },
               if self.bottom { "B" } else { "." })
    }
}

struct Board {
    pieces: [[PieceKind; 100]; 100], // rows of columns, [row][column], [y][x]
    w: usize,
    h: usize,
    groups: u16,
}

impl Board {
    fn from_stdin() -> Board {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let w = inputs[0].trim().parse::<usize>().unwrap();
        let h = inputs[1].trim().parse::<usize>().unwrap();
        let mut pieces: [[PieceKind; 100]; 100] = [[Empty; 100]; 100];

        for r in 0..h {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let row = input_line.trim_right().to_string();

            for (i,c) in row.chars().enumerate() {
                if c == '#' {
                    pieces[r][i] = Piece;
                }
            }
        }

        Board { pieces: pieces, w:w, h:h, groups: 0}
    }

    fn group(&mut self) {
        let mut gr : u16 = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                if self.pieces[y][x] == Piece {
                    self.flood_group(x, y, gr);
                    gr += 1;
                }
            }
        }
        self.groups = gr;
    }
    
    fn flood_group(&mut self, x: usize, y: usize, gr: u16) {
        let mut stack : Vec<Spot> = Vec::new();
        stack.push(Spot {x,y});
        
        while let Some(s) = stack.pop() {
            if self.needs_group_at(&s, gr) {
                self.set_group(&s, gr);
                if let Some(s2) = self.left(&s)  { if self.needs_group_at(&s2, gr) { stack.push(s2); }}
                if let Some(s2) = self.right(&s) { if self.needs_group_at(&s2, gr) { stack.push(s2); }}
                if let Some(s2) = self.up(&s)    { if self.needs_group_at(&s2, gr) { stack.push(s2); }}
                if let Some(s2) = self.down(&s)  { if self.needs_group_at(&s2, gr) { stack.push(s2); }}
            }
        }
    }
    
    fn analysis(&self) -> Vec<GroupEdges> {
        let mut edg_vec = vec![GroupEdges::default(); self.groups as usize];

        for x in 0..self.w {
            if let Group(gr) = self.pieces[0][x] {
                edg_vec[gr as usize].top = true;
            }
            if let Group(gr) = self.pieces[self.h-1][x] {
                edg_vec[gr as usize].bottom = true;
            }
        }

        for y in 0..self.h {
            if let Group(gr) = self.pieces[y][0] {
                edg_vec[gr as usize].left = true;
            }
            if let Group(gr) = self.pieces[y][self.w-1] {
                edg_vec[gr as usize].right = true;
            }
        }

        edg_vec
    }

    // Utility functions
    fn left (&self, spot: &Spot) -> Option<Spot> { if spot.x > 0        { Some(Spot{x:spot.x-1, y:spot.y,})} else { None } }
    fn up   (&self, spot: &Spot) -> Option<Spot> { if spot.y > 0        { Some(Spot{x:spot.x, y:spot.y-1,})} else { None } }
    fn right(&self, spot: &Spot) -> Option<Spot> { if spot.x+1 < self.w { Some(Spot{x:spot.x+1, y:spot.y,})} else { None } }
    fn down (&self, spot: &Spot) -> Option<Spot> { if spot.y+1 < self.h { Some(Spot{x:spot.x, y:spot.y+1,})} else { None } }

    fn set_group(&mut self, spot: &Spot, gr: u16) { self.pieces[spot.y][spot.x] = Group(gr); }
    fn needs_group_at(&self, spot: &Spot, gr: u16) -> bool {
        match self.pieces[spot.y][spot.x] {
            Empty => false,
            Piece => true,
            Group(existing_gr) => { assert_eq!(existing_gr, gr); false },
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                match self.pieces[y][x] {
                    Empty => write!(f, ".")?,
                    Piece => write!(f, "#")?,
                    Group(gr) => write!(f, "{}", gr%10)?,
                };
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

// can't be lazy evaluated before for loop -> see test nr 4 why
fn U(n: u32) -> u32 { if n == 0 { 1 } else { 2_u32.pow(n-1) } }
fn V(n: u32) -> u32 { 2_u32.pow(n) }
fn W(n: u32) -> u32 { if n == 0 { 1 } else { 2_u32.pow(n-1) + 1 }}

// see patterns.txt for explanation
fn answer(edg_vec: Vec<GroupEdges>, n: u32) -> u32 {

    let mut sum = 0;
    for (gr,edg) in edg_vec.iter().enumerate() {
        let h_factor = match (edg.left, edg.right) {
            (true, true) => 1,
            (true, false) => U(n),
            (false, false) => V(n),
            (false, true) => W(n),
        };

        let v_factor = match (edg.top, edg.bottom) {
            (true, true) => 1,
            (true, false) => U(n),
            (false, false) => V(n),
            (false, true) => W(n),
        };

        eprintln!("gr={} h={} x v={} = {}", gr, h_factor, v_factor, h_factor * v_factor);
        sum += h_factor * v_factor;
    }

    sum
}



