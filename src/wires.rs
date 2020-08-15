use std::collections::HashSet;
use std::str::FromStr;

type Pos = (isize, isize);

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    begin: Pos,
    end: Pos,
    len: isize,
    is_vert: bool,
}

impl Segment {
    pub fn new(begin: Pos, s: &str) -> Self {
        let (d, rest_bytes) = s.as_bytes().split_first().unwrap();
        let d = std::char::from_u32(*d as u32).unwrap();
        let rest = String::from_utf8(rest_bytes.to_vec()).unwrap();
        let len = isize::from_str(&rest).unwrap();

        let (end, is_vert) = match d {
            'U' => ((begin.0, begin.1 + len), true),
            'D' => ((begin.0, begin.1 - len), true),
            'R' => ((begin.0 + len, begin.1), false),
            'L' => ((begin.0 - len, begin.1), false),
            _ => panic!("invalid direction {}", d),
        };

        Self { begin, end, len, is_vert }
    }

    pub fn intersect(&self, other: &Segment) -> Option<(Pos, isize)> {
        if self.is_vert == other.is_vert {
            return None;
        }

        if self.is_vert {
            let (miny, maxy) = if self.begin.1 < self.end.1 {
                (self.begin.1, self.end.1)
            } else {
                (self.end.1, self.begin.1)
            };
            let (minx, maxx) = if other.begin.0 < other.end.0 {
                (other.begin.0, other.end.0)
            } else {
                (other.end.0, other.begin.0)
            };
            if miny <= other.begin.1 && other.begin.1 <= maxy && minx <= self.begin.0 && self.begin.0 <= maxx {
                let pos = (self.begin.0, other.begin.1);
                let delta = isize::abs(self.begin.1 - other.begin.1);
                return Some((pos, delta));
            }
        } else {
            let (minx, maxx) = if self.begin.0 < self.end.0 {
                (self.begin.0, self.end.0)
            } else {
                (self.end.0, self.begin.0)
            };
            let (miny, maxy) = if other.begin.1 < other.end.1 {
                (other.begin.1, other.end.1)
            } else {
                (other.end.1, other.begin.1)
            };
            if minx <= other.begin.0 && other.begin.0 <= maxx && miny <= self.begin.1 && self.begin.1 <= maxy {
                let pos = (other.begin.0, self.begin.1);
                let delta = isize::abs(self.begin.0 - other.begin.0);
                return Some((pos, delta));
            }
        }
        
        None
    }
}

pub fn path(segments: &[String]) -> Vec<Segment> {
    let mut pos = (0, 0);
    segments.into_iter()
        .map(|s| {
            let seg = Segment::new(pos, &s);
            pos = seg.end;
            seg
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash)]
pub struct Crossing {
    pub x: isize,
    pub y: isize,
    aint: isize,
    bint: isize,
    pub total_time: isize,
}

pub fn crossings(a: &[Segment], b: &[Segment]) -> HashSet<Crossing> {
    let mut c = HashSet::new();
    
    let mut at = 0;
    for ai in a {
        let mut bt = 0;
        for bi in b {
            match ai.intersect(bi) {
                Some(((x, y), aint)) => {
                    if x != 0 && y != 0 {
                        let (bx, by) = bi.begin;
                        let bint = manhattan_distance(x - bx, y - by);
                        c.insert(Crossing {
                            x: x,
                            y: y,
                            aint: at + aint,
                            bint: bt + bint,
                            total_time: at + aint + bt + bint,
                        });
                    }
                },
                None => (),
            }
            bt += bi.len;
        }
        at += ai.len;
    }
    
    c
}

pub fn manhattan_distance(x: isize, y: isize) -> isize {
    isize::abs(x) + isize::abs(y)
}
