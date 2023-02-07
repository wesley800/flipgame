#![allow(non_snake_case)]

use std::collections::HashMap;

#[repr(i8)]
#[derive(PartialEq, Eq, Clone, Copy)]
enum Node {
    OutOfBound = -1,
    Off = 0,
    On = 1,
    NotCare = 2,
}
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}
struct Board {
    w: usize,
    h: usize,
    m: Vec<Node>,
}
impl Board {
    fn mget(&self, x: isize, y: isize) -> Node {
        return if x >= 0 && x < self.h as isize && y >= 0 && y < self.w as isize {
            self.m[(x * self.w as isize + y) as usize]
        } else {
            Node::OutOfBound
        };
    }
    fn gen_matrix(&self) -> (Vec<Position>, nalgebra::DMatrix<i8>, nalgebra::DVector<i8>) {
        let (mapp2n, mapn2p) = {
            let mut mapn2p = Vec::<Position>::new();
            let mut mapp2n = HashMap::<Position, usize>::new();
            for x in 0..self.h {
                for y in 0..self.w {
                    match self.mget(x as isize, y as isize) {
                        Node::On | Node::Off => {
                            let t = Position { x, y };
                            mapp2n.insert(t, mapn2p.len());
                            mapn2p.push(t);
                        }
                        _ => {}
                    }
                }
            }
            (mapp2n, mapn2p)
        };
        let DELTAS = [(0, -1), (0, 1), (1, 0), (-1, 0)];
        let (ma, vx) = {
            let count = mapp2n.len();
            let mut ma = nalgebra::DMatrix::<i8>::zeros(count, count);
            let mut vx = nalgebra::DVector::<i8>::zeros(count);
            for (n, &pos) in mapn2p.iter().enumerate() {
                let (x, y) = (pos.x as isize, pos.y as isize);
                ma[(n, n)] = 1;
                match self.mget(x, y) {
                    Node::On => vx[n] = 1,
                    Node::Off => {}
                    _ => panic!(),
                }
                for delta in DELTAS {
                    let (xx, yy) = (x + delta.0, y + delta.1);
                    //TODO: potential panic here(at .unwrap()). Think again if NotCare is needed?
                    //Of course it is. That's why the matrix can be not square.
                    if self.mget(xx, yy) != Node::OutOfBound {
                        let &nn = mapp2n
                            .get(&Position {
                                x: xx as usize,
                                y: yy as usize,
                            })
                            .unwrap();
                        ma[(n, nn)] = 1;
                    }
                }
            }
            (ma, vx)
        };
        return (mapn2p, ma, vx);
    }

    fn from_file(filename: &str) -> Self {
        let f = std::fs::File::open(filename).expect("Failed to read input.txt file");
        let rd = std::io::BufReader::new(f);
        let mut data = Vec::<Node>::new();
        let (mut w, mut h) = (-1 as isize, 0 as isize);
        for (lineno, line) in std::io::BufRead::lines(rd).enumerate() {
            let line = line
                .unwrap_or_else(|e| panic!("Error reading {}th line of input. err={}", lineno, e));
            let old_len = data.len();
            for c in line.chars() {
                data.push(match c {
                    '0' => Node::Off,
                    '1' => Node::On,
                    'T' | 't' => Node::OutOfBound,
                    'G' | 'g' => Node::NotCare,
                    ' ' | '\r' | '\n' => continue,
                    _ => panic!("Error parsing input file line {}, char={}", lineno, c),
                });
            }
            if w == -1 {
                w = (data.len() - old_len) as isize;
            } else if data.len() - old_len != w as usize {
                panic!(
                    "Line {} has length of {}, rather than {}",
                    lineno,
                    data.len() - old_len,
                    w
                );
            }
            h += 1;
        }
        return Self {
            w: w as usize,
            h: h as usize,
            m: data,
        };
    }
}

#[allow(dead_code)]
fn compare_DMatrix_row<T>(A: &nalgebra::DMatrix<T>, a: usize, b: usize) -> std::cmp::Ordering
where
    T: std::cmp::Ord + Copy,
{
    for i in 0..A.ncols() {
        let (aa, bb) = (A[(a, i)], A[(b, i)]);
        if aa < bb {
            return std::cmp::Ordering::Less;
        } else if aa > bb {
            return std::cmp::Ordering::Greater;
        }
    }
    return std::cmp::Ordering::Equal;
}

#[allow(dead_code)]
mod newtype {
    use nalgebra::*;
    pub type DRowVector<T> = Matrix<T, U1, Dyn, VecStorage<T, U1, Dyn>>;
}

fn print_mat<T, R, C, S, TI>(A: &nalgebra::Matrix<T, R, C, S>, ind: TI)
where
    T: std::fmt::Display,
    C: nalgebra::Dim,
    R: nalgebra::Dim,
    S: nalgebra::RawStorage<T, R, C>,
    TI: Iterator<Item = usize>,
{
    //return;
    for i in ind {
        print!("{:3}: ", i);
        for j in 0..A.ncols() {
            print!("{}", A[(i, j)]);
        }
        println!("");
    }
}

fn solve_linear_formulae(ma0: nalgebra::DMatrix<i8>, vx0: nalgebra::DVector<i8>) -> Vec<i8> {
    let mut ma = nalgebra::DMatrix::<i8>::zeros(ma0.nrows(), ma0.ncols() + 1);

    /* // ma_ind must be updated repeatly.
    let ma_ind = {
        let mut t = (0..ma0.nrows()).collect::<Vec<usize>>();
        t.sort_by(|&a, &b| compare_DMatrix_row(&ma0, a, b).reverse());
        t
    };*/

    let mut ma_ind: Vec<usize> = (0..ma.nrows()).collect();

    // ma=hcat(ma0,vx0)
    for i in 0..ma0.nrows() {
        ma.row_part_mut(i, ma0.ncols()).copy_from(&ma0.row(i));
        ma[(i, ma0.ncols())] = vx0[i];
    }

    println!("original mat");
    print_mat(&ma, ma_ind.iter().map(|x| *x));

    let mut cur_prefix: usize = 0;
    let succeeded = 'fail_underrank: loop {
        for k in 0..ma.nrows() {
            // Search for longest prefix line, and swap it to `md_ind[k]`
            // On GF(2) we don't need this actually.
            //let max_factor:i8=i8::MIN;
            let cur_row = 'outer: loop {
                for ii in k..ma.nrows() {
                    let i = ma_ind[ii];
                    if ma[(i, cur_prefix)] != 0 {
                        ma_ind.swap(ii, k);
                        break 'outer i;
                    }
                }
                cur_prefix += 1;
                if cur_prefix == ma.ncols() {
                    break 'fail_underrank Err(());
                }
            };

            let invinds: Vec<usize> = (cur_prefix..ma.ncols())
                .filter(|&x| ma[(cur_row, x)] == 1)
                .collect();
            for &i in ma_ind[k + 1..].iter() {
                if ma[(i, cur_prefix)] != 0 {
                    for &j in invinds.iter() {
                        ma[(i, j)] ^= 1; //GF(2) here!
                    }
                }
            }

            cur_prefix += 1;

            println!("{k} mat, found {cur_row}. cur_prefix={cur_prefix}");
            //print_mat(&ma, ma_ind.iter().map(|&x| x));
        }
        break 'fail_underrank Ok(());
    };
    println!("Succeeded={succeeded:?}");
    print_mat(&ma, ma_ind.iter().map(|&x| x));

    if succeeded.is_ok() {
        let mut xs = vec![0; ma.nrows()];
        for (kk, &k) in ma_ind.iter().enumerate().rev() {
            xs[kk]=/*0^*/ma[(k,ma.ncols()-1)]; // GF(2)
            for i in kk + 1..ma.ncols() - 1 {
                if ma[(k, i)] != 0 {
                    // GF(2)
                    xs[kk] ^= xs[i];
                }
            }
        }
        return xs;
    } else {
        return vec![];
    }
}

fn main() {
    let board = Board::from_file("input.txt");
    println!("Board w={} h={}", board.w, board.h);

    let (mapn2p, A, x) = board.gen_matrix();
    for (i, m) in mapn2p.iter().enumerate() {
        println!("map{:3}: {:2} {:2}", i, m.x, m.y);
    }
    let ans = solve_linear_formulae(A, x);
    println!("ans={ans:?}");
    for (i, &x) in ans.iter().enumerate() {
        if x != 0 {
            println!("{},{}", mapn2p[i].x, mapn2p[i].y);
        }
    }
}
