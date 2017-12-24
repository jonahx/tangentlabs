/// generate the the C(n,k) possible partitions you can create
/// by splitting a set of n items into subsets of length k and n-k

use std::clone;

struct PartitionGenerator<T: clone::Clone> {
  k: usize,  xs: Vec<T>,
  i: usize,  state: u8,  // internal flow control stuff
  held: Option<T>,       // held and subs are for recursion support
  subs: Option<Box<PartitionGenerator<T>>>
}

impl<T: clone::Clone> Iterator for PartitionGenerator<T> {
  // this is basically the ugly state machine rust would generate if it had async/yield
  // ... which it probably will at some point:
  //    https://internals.rust-lang.org/t/help-test-async-await-generators-coroutines/5835
  // but this was a fun (and/or painful) learning exercise, so whatever.
  type Item = (Vec<T>, Vec<T>);
  fn next(&mut self) -> Option<Self::Item> {
    let n = self.xs.len();
    loop { match self.state {
      0 => {
        if self.k==0 { self.state = 255; return Some((vec![], self.xs.clone())) }
        if self.k==n { self.state = 255; return Some((self.xs.clone(), vec![])) }
        // k < nx so nx can't be 0
        self.state = 1 }
      1 => { // top of outer loop -- for i in 0..(nx-k)
        if self.i <= n-self.k {
          // add item[i] to the left subset
          self.held = Some(self.xs[self.i].clone());
          // and recursively generate sub-partitions of size k-1 from the remaining items:
          let mut tail = self.xs.clone(); tail.remove(self.i);
          self.subs = Some(Box::new(partitions_i(self.k-1, tail, self.i)));
          self.i += 1; self.state = 2
        } else { self.state = 255 }}
      2 => { // inner loop: for (aa,bb) in the sub-partitions
        match self.subs {
          None => panic!("error! self.subs should never be None in state 2!!"),
          Some(ref mut pg) => {
            match pg.next() {
              Some((mut aa,bb)) => {
                aa.insert(0, self.held.clone().unwrap());
                return Some((aa, bb)) }
              None => { self.state = 1 }}}  // end of inner loop  
        }}
      255 => return None,  // "done" state
      _ => panic!("PartitionGenerator entered unknown state: {}", self.state)
    }}
  }
}

fn partitions_i<T:clone::Clone>(k:usize, xs:Vec<T>, i:usize) -> PartitionGenerator<T> {
  // internal helper function. we want distinct combinations, not all permutations,
  // so when we do the sub-partitions, we pass in the current index. this ensures
  // that the sub-partitions don't include items we've already held in previous
  // iterations of the outer loop.
  PartitionGenerator{k:k, xs:xs, i:i, state:0, held:None, subs:None}
}

fn partitions<T:clone::Clone>(k:usize, xs:Vec<T>) -> PartitionGenerator<T> {
  partitions_i(k, xs, 0)
}

fn main() {
  for (i, p) in partitions(2, (0..4).collect()).enumerate() {
    println!("{}: {:?}", i, p)   
  }
}
