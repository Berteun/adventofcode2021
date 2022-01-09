use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;

#[derive(Debug)]
enum Node {
    Lit(Literal),
    Pair(Pair),
}

type Tree = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Pair {
    p: Option<Tree>,
    l: Tree,
    r: Tree,
}

#[derive(Debug)]
struct Literal {
    value: i32,
    parent: Option<Tree>,
    prev: Option<Tree>,
    next: Option<Tree>,
}

impl Node {
    fn depth(&self) -> i32 {
        match &self {
            Node::Lit(l) => l.parent.as_ref().unwrap().borrow().depth(),
            Node::Pair(p) => match &p.p {
                Some(parent) => 1 + parent.borrow().depth(),
                None => 0,
            },
        }
    }

    fn print(&self, depth: usize) {
        match &self {
            Node::Pair(p) => {
                p.l.borrow().print(depth + 1);
                p.r.borrow().print(depth + 1);
            }
            Node::Lit(l) => println!("{}{}", "\t".repeat(depth), l.value),
        }
    }
}

fn add(left: Tree, right: Tree) -> Tree {
    Rc::new(RefCell::new(Node::Pair(Pair {
        l: left,
        r: right,
        p: None,
    })))
}

fn parent(t: &Tree) -> Tree {
    let p = match &*t.borrow() {
        Node::Lit(l) => l.parent.clone(),
        Node::Pair(pair) => pair.p.clone(),
    };
    p.unwrap().clone()
}

fn explode(t: &mut Tree) {
    let (l, r, parent) = match &*t.borrow() {
        Node::Pair(p) => (p.l.clone(), p.r.clone(), p.p.clone()),
        _ => panic!("Wrong explode!"),
    };

    let (lv, prev) = match &*l.borrow() {
        Node::Lit(l) => (l.value, l.prev.clone()),
        _ => panic!("Wrong explode!"),
    };

    let (rv, next) = match &*r.borrow() {
        Node::Lit(r) => (r.value, r.next.clone()),
        _ => panic!("Wrong explode!"),
    };

    let new_tree = Rc::new(RefCell::new(Node::Lit(Literal {
        value: 0,
        prev: prev.clone(),
        next: next.clone(),
        parent: parent.clone(),
    })));

    if let Some(prev_l) = prev.clone() {
        match &mut *prev_l.borrow_mut() {
            Node::Lit(prev_ll) => {
                prev_ll.value += lv;
                prev_ll.next = Some(new_tree.clone());
            }
            _ => panic!("Wrong explode!"),
        }
    }

    if let Some(next_l) = next.clone() {
        match &mut *next_l.borrow_mut() {
            Node::Lit(next_ll) => {
                next_ll.value += rv;
                next_ll.prev = Some(new_tree.clone());
            }
            _ => panic!("Wrong explode!"),
        }
    }

    let parent_node = parent.unwrap();
    match &mut *parent_node.borrow_mut() {
        Node::Pair(p) => {
            if Rc::ptr_eq(&p.l, t) {
                p.l = new_tree;
            } else {
                p.r = new_tree;
            }
        }
        _ => panic!("Incorrect tree!"),
    };
}

fn split(t: &mut Tree) {
    let (nv, prev, next, parent) = match &*t.borrow() {
        Node::Lit(l) => (l.value, l.prev.clone(), l.next.clone(), l.parent.clone()),
        _ => panic!("Wrong split!"),
    };

    let lv = nv / 2;
    let rv = (nv + 1) / 2;

    let new_left = Rc::new(RefCell::new(Node::Lit(Literal {
        value: lv,
        parent: None,
        prev: prev.clone(),
        next: None,
    })));

    let new_right = Rc::new(RefCell::new(Node::Lit(Literal {
        value: rv,
        parent: None,
        prev: Some(new_left.clone()),
        next: next.clone(),
    })));

    let new_tree = Rc::new(RefCell::new(Node::Pair(Pair {
        p: parent.clone(),
        l: new_left.clone(),
        r: new_right.clone(),
    })));

    let parent_node = parent.clone().unwrap();
    match &mut *parent_node.borrow_mut() {
        Node::Pair(p) => {
            if Rc::ptr_eq(&p.l, t) {
                p.l = new_tree;
            } else {
                p.r = new_tree;
            }
        }
        _ => panic!("Incorrect tree!"),
    };
}

fn find_explodable(t: &Tree, depth: usize) -> Option<Tree> {
    match &*t.borrow() {
        Node::Lit(l) => None,
        Node::Pair(p) => {
            if depth == 4 {
                return Some(t.clone());
            }
            let n = find_explodable(&p.l, depth + 1);
            if n.is_none() {
                find_explodable(&p.r, depth + 1)
            } else {
                n
            }
        }
    }
}

fn find_splittable(t: &Tree) -> Option<Tree> {
    match &*t.borrow() {
        Node::Lit(l) => {
            if l.value > 9 {
                Some(t.clone())
            } else {
                None
            }
        }
        Node::Pair(p) => {
            let n = find_splittable(&p.l);
            if n.is_none() {
                find_splittable(&p.r)
            } else {
                n
            }
        }
    }
}

fn add_and_reduce(left: Tree, right: Tree) -> Tree {
    let nt = add(left, right);
    loop {
        link_nodes(nt.clone(), None);
        let expl_node = find_explodable(&nt, 0);
        if let Some(mut t) = expl_node {
            explode(&mut t);
            continue;
        }
        let split_node = find_splittable(&nt);
        if let Some(mut t) = split_node {
            split(&mut t);
            continue;
        }
        break;
    }
    nt
}

fn parse_element(line: &mut Peekable<std::str::Chars>) -> Tree {
    let expect_next = |l: &mut Peekable<_>, c| assert_eq!(l.next().unwrap(), c);
    let parse_elem = |l: &mut Peekable<_>| match l.peek() {
        Some('[') => parse_element(l),
        Some('0'..='9') => Rc::new(RefCell::new(Node::Lit(Literal {
            value: l.next().unwrap() as i32 - '0' as i32,
            prev: None,
            next: None,
            parent: None,
        }))),
        _ => panic!("Unexpected input"),
    };

    expect_next(line, '[');
    let l = parse_elem(line);
    expect_next(line, ',');
    let r = parse_elem(line);

    expect_next(line, ']');
    Rc::new(RefCell::new(Node::Pair(Pair { l, r, p: None })))
}

fn collect_leaves(e: &Tree) -> Vec<Tree> {
    match &*e.borrow() {
        Node::Lit(_) => vec![e.clone()],
        Node::Pair(p) => {
            let mut res = collect_leaves(&p.l);
            res.append(&mut collect_leaves(&p.r));
            res
        }
    }
}

fn link_parents(e: &Tree, parent: Option<Tree>) {
    match &mut *e.borrow_mut() {
        Node::Lit(l) => l.parent = parent,
        Node::Pair(pair) => {
            pair.p = parent;
            link_parents(&pair.l, Some(e.clone()));
            link_parents(&pair.r, Some(e.clone()));
        }
    }
}

fn link_nodes(e: Tree, parent: Option<Tree>) -> Tree {
    link_parents(&e, parent);
    let leaves = collect_leaves(&e);
    let mut prev: Option<Tree> = None;
    for leaf in leaves {
        if let Node::Lit(l) = &mut *leaf.borrow_mut() {
            if let Some(p) = &prev {
                if let Node::Lit(pl) = &mut *p.borrow_mut() {
                    pl.next = Some(leaf.clone());
                }
            }

            l.prev = prev.clone();
            prev = Some(leaf.clone());
        }
    }
    e
}

fn parse(s: &str) -> Tree {
    let t = parse_element(&mut s.chars().peekable());
    link_nodes(t, None)
}

fn read_input() -> Vec<Tree> {
    include_str!("../input").lines().map(parse).collect()
}

fn magnitude(t: &Tree) -> i64 {
    match &*t.borrow() {
        Node::Lit(l) => l.value as i64,
        Node::Pair(p) => 3 * magnitude(&p.l) + 2 * magnitude(&p.r),
    }
}

fn part1(inp: Vec<Tree>) -> i64 {
    let sum = inp.into_iter().reduce(|l, r| add_and_reduce(l, r));
    magnitude(&sum.unwrap())
}

fn part2() -> i64 {
    let n = read_input().len();
    let mut max_magn = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let inp = read_input();
            max_magn = std::cmp::max(
                max_magn,
                magnitude(&add_and_reduce(inp[i].clone(), inp[j].clone())),
            )
        }
    }
    max_magn
}

fn leaf_value(n: &Tree) -> String {
    match &*n.borrow() {
        Node::Lit(l) => l.value.to_string(),
        Node::Pair(_) => String::from("-"),
    }
}

fn print_leaves(n: &Tree) {
    let leaves = collect_leaves(n);
    for leaf in leaves {
        match &*leaf.borrow() {
            Node::Lit(l) => {
                println!(
                    "{}, p: {}, n: {}, d: {}",
                    l.value,
                    match &l.prev {
                        None => String::from("-"),
                        Some(p) => leaf_value(&p),
                    },
                    match &l.next {
                        None => String::from("-"),
                        Some(p) => leaf_value(&p),
                    },
                    leaf.borrow().depth(),
                );
            }
            _ => {}
        }
    }
}

fn main() {
    let tree = read_input();
    println!("{}", part1(read_input()));
    println!("{}", part2());
}

#[test]
fn test_magnitude_1() {
    let t = parse("[[1,2],[[3,4],5]]");
    assert_eq!(magnitude(&t), 143);
}

#[test]
fn test_magnitude_2() {
    let t = parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    assert_eq!(magnitude(&t), 3488);
}

#[test]
fn test_explode_1() {
    let t = parse("[[[[[9,8],1],2],3],4]");
    let mut explodable = find_explodable(&t, 0).unwrap();
    explode(&mut explodable);
    let u = parse("[[[[0,9],2],3],4]");
    assert_eq!(magnitude(&t), magnitude(&u));
}
