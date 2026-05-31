// TODO: not entirely finished
// I should aff increment function
use std::{borrow::Cow::Borrowed, cmp::max, collections::HashMap};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn read_to_hash_map(lx: Option<&ListNode>) -> HashMap<usize, i32> {
    // let mut lx = lx.as_deref();
    let mut lx = lx;
    let mut hm = HashMap::<usize, i32>::new();
    let mut counter = 0;
    while let Some(_) = lx {
        hm.insert(counter, lx.unwrap().val);
        counter += 1;
        lx = lx.unwrap().next.as_deref();
    }
    hm
}

fn print_list_node(head: &ListNode) {
    let mut current = Some(head);
    print!("[");
    while let Some(_) = current {
        print!("{}, ", current.unwrap().val);
        current = current.unwrap().next.as_deref();
    }
    println!("]");
}

// TODO: instead that change it into increment current
fn increment_next(current: &mut ListNode) {
    match current.next {
        Some(_) => {
            let next_val = current.next.as_deref().unwrap().val;
            if next_val < 9 {
                current.next.as_deref_mut().unwrap().val += 1;
            } else {
                increment_next(current.next.as_deref_mut().unwrap());
            }
        }
        None => {
            current.next = Some(Box::new(ListNode::new(1)));
        }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut hm1 = read_to_hash_map(l1.as_deref());
    let mut hm2 = read_to_hash_map(l2.as_deref());
    let max_len = max(hm1.len(), hm2.len());
    // let mut hm_out = HashMap::<usize, i32>::new();
    let mut ln_out_first = ListNode::new(0);
    let mut ln_out_current = &mut ln_out_first;
    // let mut current_ln = ListNode::new(0);
    // let mut carry = 0;
    for m_pow in 0..max_len {
        let sum = hm1.get(&m_pow).unwrap_or(&0) + hm2.get(&m_pow).unwrap_or(&0);
        let digit = sum % 10;
        let carry = sum / 10;
        if carry > 0 {
            increment_next(ln_out_current);
        }
        // hm_out.insert(m_pow, digit);
        // This is bad idea tu begin with, it doesn't taks into an account that some val may already exists because previous sum was more than 9
        match ln_out_current.next {
            Some(_) => {}
            None => {
                let mut current_ln = ListNode::new(digit);
                ln_out_current.next = Some(Box::new(current_ln));
            }
        }
        ln_out_current = ln_out_current.next.as_deref_mut().unwrap();
    }

    // println!("hm_out: {hm_out:?}");

    return ln_out_first.next;
    // return Some(Box::new(ListNode::new(0)))
}

pub fn test1() {
    let mut l1a = ListNode::new(2);
    let mut l1b = ListNode::new(4);
    let mut l1c = ListNode::new(3);
    l1b.next = Some(Box::new(l1c));
    l1a.next = Some(Box::new(l1b));

    let mut l2a = ListNode::new(5);
    let mut l2b = ListNode::new(6);
    let mut l2c = ListNode::new(4);
    l2b.next = Some(Box::new(l2c));
    l2a.next = Some(Box::new(l2b));

    print_list_node(&l1a);

    let hm = read_to_hash_map(Some(&l1a));
    println!("hm: {hm:?}");

    let res = add_two_numbers(Some(Box::new(l1a)), Some(Box::new(l2a)));
    match res {
        Some(ll) => {
            print_list_node(ll.as_ref());
        }
        None => {
            println!("Empty result");
        }
    }
}

pub fn test2() {
    let mut l1a = ListNode::new(9);
    let mut l1b = ListNode::new(9);
    let mut l1c = ListNode::new(9);
    let mut l1d = ListNode::new(9);
    let mut l1e = ListNode::new(9);
    let mut l1f = ListNode::new(9);
    let mut l1g = ListNode::new(9);
    l1f.next = Some(Box::new(l1g));
    l1e.next = Some(Box::new(l1f));
    l1d.next = Some(Box::new(l1e));
    l1c.next = Some(Box::new(l1d));
    l1b.next = Some(Box::new(l1c));
    l1a.next = Some(Box::new(l1b));

    let mut l2a = ListNode::new(9);
    let mut l2b = ListNode::new(9);
    let mut l2c = ListNode::new(9);
    let mut l2d = ListNode::new(9);
    l2c.next = Some(Box::new(l2d));
    l2b.next = Some(Box::new(l2c));
    l2a.next = Some(Box::new(l2b));

    let res = add_two_numbers(Some(Box::new(l1a)), Some(Box::new(l2a)));
    match res {
        Some(ll) => {
            print_list_node(ll.as_ref());
        }
        None => {
            println!("Empty result");
        }
    }
}
