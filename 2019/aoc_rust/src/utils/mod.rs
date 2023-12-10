use std::sync::mpsc::{Receiver, Sender};

pub struct Computer {
    pub p: Vec<isize>,
    pub n: isize,
    pub rb: isize,
    pub i: Sender<isize>,
    pub o: Receiver<isize>,
    pub _i: Receiver<isize>,
    pub _o: Sender<isize>,
}
