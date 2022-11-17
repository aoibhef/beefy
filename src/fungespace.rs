use std::fmt;

use crate::instruction_pointer::InstructionPointer;

pub struct FungeSpace {
    px_py: Vec<Vec<i64>>, // +x, +y (quadrant 4); x >= 0 && y >= 0
    nx_py: Vec<Vec<i64>>, // -x, +y (quadrant 3); x <  0 && y >= 0
    nx_ny: Vec<Vec<i64>>, // -x, -y (quadrant 2); x <  0 && y <  0
    px_ny: Vec<Vec<i64>>, // +x, -y (quadrant 1); x >= 0 && y <  0

    min_point: (i64, i64),
    max_point: (i64, i64),
}

impl FungeSpace {
    pub fn from_file(path: &str) -> FungeSpace {
        let px_py = if let Ok(v) = std::fs::read(path) {
            let v = v
                .into_iter()
                .filter_map(|e| (e != 12 && e != 13).then(|| e as i64))
                .collect();
            let v = split_vec_delimited(v, &10);
            v.into_iter()
                .map(|mut e| loop {
                    if e.last().map_or(false, |n| n == &32) {
                        e.pop();
                    } else {
                        break e;
                    }
                })
                .collect::<Vec<Vec<i64>>>()
        } else {
            panic!("failed to read file")
        };

        let max_point = (
            px_py
                .iter()
                .max_by(|r1, r2| r1.len().cmp(&r2.len()))
                .unwrap()
                .len() as i64
                - 1,
            px_py.len() as i64 - 1,
        );

        // Initially, we use the loaded data to initialize the +x, +y quadrant,
        // and the others are just empty vectors for if we ever try and use the
        // negative space
        FungeSpace {
            px_py,
            nx_py: vec![],
            nx_ny: vec![],
            px_ny: vec![],

            min_point: (0, 0),
            max_point,
        }
    }

    pub fn instruction_at_ip(&self, ip: &InstructionPointer) -> i64 {
        self.instruction_at_pos(ip.pos)
    }

    pub fn instruction_at_pos(&self, pos: (i64, i64)) -> i64 {
        match pos {
            (x, y) if x >= 0 && y >= 0 => {
                let x: usize = x.try_into().expect("invalid xpos");
                let y: usize = y.try_into().expect("invalid ypos");
                self.px_py.get(y).map_or(32, |v| *v.get(x).unwrap_or(&32))
            }
            (x, y) if x < 0 && y >= 0 => {
                let x: usize = x.abs().try_into().expect("invalid xpos");
                let y: usize = y.try_into().expect("invalid ypos");
                self.nx_py
                    .get(y)
                    .map_or(32, |v| *v.get(x - 1).unwrap_or(&32))
            }
            (x, y) if x < 0 && y < 0 => {
                let x: usize = x.abs().try_into().expect("invalid xpos");
                let y: usize = y.abs().try_into().expect("invalid ypos");
                self.nx_ny
                    .get(y - 1)
                    .map_or(32, |v| *v.get(x - 1).unwrap_or(&32))
            }
            (x, y) if x >= 0 && y < 0 => {
                let x: usize = x.try_into().expect("invalid xpos");
                let y: usize = y.abs().try_into().expect("invalid ypos");
                self.px_ny
                    .get(y - 1)
                    .map_or(32, |v| *v.get(x).unwrap_or(&32))
            }
            (_, _) => unreachable!(),
        }
    }

    pub fn min_point(&self) -> (i64, i64) {
        self.min_point
    }

    pub fn max_point(&self) -> (i64, i64) {
        self.max_point
    }

    pub fn in_bounds(&self, ip: &InstructionPointer) -> bool {
        return ip.pos.0 >= self.min_point.0
            && ip.pos.0 <= self.max_point.0
            && ip.pos.1 >= self.min_point.1
            && ip.pos.1 <= self.max_point.1;
    }
}

impl fmt::Display for FungeSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for y in self.min_point.1..=self.max_point.1 {
            if y != self.min_point.1 {
                write!(f, " ")?;
            }
            write!(f, "[")?;

            for x in self.min_point.0..=self.max_point.0 {
                let v = match self.instruction_at_pos((x, y)) {
                    v if v <= 0 => char::REPLACEMENT_CHARACTER,
                    v @ _ => char::from_u32(v as u32).unwrap_or(char::REPLACEMENT_CHARACTER),
                };
                write!(f, "{}", v)?;
            }

            write!(f, "]")?;
            if y != self.max_point.1 {
                writeln!(f)?;
            }
        }
        write!(f, "]")
    }
}

pub fn split_vec_delimited<T>(input: Vec<T>, delim: &T) -> Vec<Vec<T>>
where
    T: PartialEq<T> + std::clone::Clone,
{
    let elems = input.iter().enumerate();
    let (k, mut r) = elems.fold((0, vec![]), |(i, mut r), (j, x)| {
        if x == delim && j > 0 {
            r.push(input[i..j].to_vec());
            (j + 1, r)
        } else {
            (i, r)
        }
    });
    if !input.is_empty() {
        r.push(input[k..].to_vec())
    }
    r
}
