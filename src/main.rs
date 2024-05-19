use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use core::cmp::Ordering;

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    freq: (char, i32),
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

#[allow(dead_code)]
fn make_node(key: char, value: i32) -> Node {
    Node { freq: (key, value), left: None, right: None }
}

fn find_codes(heap: BinaryHeap<Reverse<Node>>) {
    println!("{:#?}", heap);
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.1.cmp(&other.freq.1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq.1 == other.freq.1
    }
}

impl Eq for Node {}

fn main() {

    let mut freqs: HashMap<char, i32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let input: String = String::from("abcdbbaadaa");

    for c in input.chars() {
        freqs.entry(c).and_modify(|c| *c += 1).or_insert(1);
    };

    for (key, value) in &freqs {
        println!("{key}: {value}");
        let n = make_node(*key, *value);
        heap.push(Reverse(n));
    }

    while heap.len() != 1 {
        let left = heap.pop().unwrap().0;
        let right = heap.pop().unwrap().0;
        let mut parent = make_node('P', left.freq.1 + right.freq.1);
        parent.left = Some(Box::new(left));
        parent.right = Some(Box::new(right));
        heap.push(Reverse(parent));
    }

    find_codes(heap);
}
