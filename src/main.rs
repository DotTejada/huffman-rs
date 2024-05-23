use std::collections::HashMap;
use std::collections::BinaryHeap;
use core::cmp::Ordering;

#[derive(Debug, Clone)]
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

fn is_leaf(n: &Node) -> bool {
    if n.left.is_none() && n.right.is_none() { return true } else { return false }
}

fn traverse(n: Node) -> Vec<(char, i32, &'static str)> {
    let mut stack: Vec<Node> = vec![n];
    let mut result = Vec::new();
    while stack.len() > 0 {
        let current = stack.pop().unwrap();

        if is_leaf(&current) { 
            result.push((current.freq.0, current.freq.1, "leaf")) 
        } else {
            result.push((current.freq.0, current.freq.1, "parent")) 
        }
        println!("{:?}: {:?}", current.freq.0, current.freq.1);

        if current.right.is_some() { stack.push(*current.right.unwrap()) }
        if current.left.is_some() { stack.push(*current.left.unwrap()) }
    }
    return result
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.1.cmp(&self.freq.1)
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
    //let input: String = String::from("avadakadavrabbbbbb");
    let input: String = String::from("abcdbbaadaa");

    for c in input.chars() {
        freqs.entry(c).and_modify(|c| *c += 1).or_insert(1);
    };

    for (key, value) in &freqs {
        //println!("{key}: {value}");
        let n = make_node(*key, *value);
        heap.push(n);
    }

    //println!("{:#?}", heap);

    while heap.len() != 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let mut parent = make_node('P', left.freq.1 + right.freq.1);
        parent.left = Some(Box::new(left));
        parent.right = Some(Box::new(right));
        heap.push(parent);
    }
    //println!("{:#?}", heap);
    let tree = traverse(heap.pop().unwrap());
    println!("{:?}", tree);

    let mut i: usize = 1;
    let mut lcount: u8 = 0;
    let mut pcount: u8 = 0;
    let mut code = String::new();
    while i < tree.len() {
        match tree[i].2 {
            "parent" => { 
                pcount += 1;
                if pcount == 1 {
                    code.push('0'); 
                    i += 1; 
                } else {
                }
                lcount = 0;
            },
            "leaf" => { 
                lcount += 1;
                if lcount == 1 {
                    code.push('0'); 
                } else {
                    code.push('1'); 
                }
                println!("{:?}: {:?}", tree[i].0, code); 
                code.pop(); 
            },
            _ => unreachable!()
        }
    }
}
