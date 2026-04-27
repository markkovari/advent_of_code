use std::sync::mpsc::{channel, Receiver, Sender};

pub mod intcode;

pub struct Computer {
    pub p: Vec<isize>,
    pub n: isize,
    pub rb: isize,
    pub i: Sender<isize>,
    pub o: Receiver<isize>,
    _i: Receiver<isize>,
    _o: Sender<isize>,
}

impl Clone for Computer {
    fn clone(&self) -> Self {
        let ((i, _i), (_o, o)) = (channel(), channel());
        Self {
            p: self.p.clone(),
            n: self.n,
            rb: self.rb,
            i,
            o,
            _i,
            _o,
        }
    }
}

impl Computer {
    pub fn from_string(s: &str) -> Self {
        let ((i, _i), (_o, o)) = (channel(), channel());
        Self {
            p: s.split(',').filter_map(|l| l.trim().parse().ok()).collect(),
            n: 0,
            rb: 0,
            i,
            o,
            _i,
            _o,
        }
    }

    #[must_use]
    #[inline(always)]
    fn acc(&mut self, i: isize, m: Option<isize>) -> &mut isize {
        let i = match m {
            Some(0) | None => self.p[i as usize] as usize,
            Some(1) => i as usize,
            Some(2) => (self.rb + self.p[i as usize]) as usize,
            _ => unreachable!(),
        };
        if i >= self.p.len() {
            self.p.resize(i + 1, 0);
        }
        &mut self.p[i]
    }

    pub fn run(&mut self) -> bool {
        loop {
            let mut inst_val = self.p[self.n as usize];
            let opcode = inst_val % 100;
            inst_val /= 100;
            let mut modes = Vec::new();
            while inst_val > 0 {
                modes.push(inst_val % 10);
                inst_val /= 10;
            }

            self.n = match opcode {
                1 => {
                    let v = *self.acc(self.n + 1, modes.get(0).copied())
                        + *self.acc(self.n + 2, modes.get(1).copied());
                    *self.acc(self.n + 3, modes.get(2).copied()) = v;
                    self.n + 4
                }
                2 => {
                    let v = *self.acc(self.n + 1, modes.get(0).copied())
                        * *self.acc(self.n + 2, modes.get(1).copied());
                    *self.acc(self.n + 3, modes.get(2).copied()) = v;
                    self.n + 4
                }
                3 => {
                    match self._i.try_recv() {
                        Ok(i) => *self.acc(self.n + 1, modes.get(0).copied()) = i,
                        Err(_) => return false,
                    }
                    self.n + 2
                }
                4 => {
                    let v = *self.acc(self.n + 1, modes.get(0).copied());
                    self._o.send(v).unwrap();
                    self.n + 2
                }
                5 if *self.acc(self.n + 1, modes.get(0).copied()) != 0 => {
                    *self.acc(self.n + 2, modes.get(1).copied()) as isize
                }
                5 => self.n + 3,
                6 if *self.acc(self.n + 1, modes.get(0).copied()) == 0 => {
                    *self.acc(self.n + 2, modes.get(1).copied()) as isize
                }
                6 => self.n + 3,
                7 => {
                    let v = if *self.acc(self.n + 1, modes.get(0).copied())
                        < *self.acc(self.n + 2, modes.get(1).copied())
                    {
                        1
                    } else {
                        0
                    };
                    *self.acc(self.n + 3, modes.get(2).copied()) = v;
                    self.n + 4
                }
                8 => {
                    let v = if *self.acc(self.n + 1, modes.get(0).copied())
                        == *self.acc(self.n + 2, modes.get(1).copied())
                    {
                        1
                    } else {
                        0
                    };
                    *self.acc(self.n + 3, modes.get(2).copied()) = v;
                    self.n + 4
                }
                9 => {
                    self.rb += *self.acc(self.n + 1, modes.get(0).copied());
                    self.n + 2
                }
                99 => return true,
                _ => panic!("Unknown OPCODE: {} at {}", self.p[self.n as usize], self.n),
            };
        }
    }
}
