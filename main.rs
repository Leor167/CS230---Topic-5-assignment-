use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;



fn main() { 
    let user_list = Arc::new(Mutex::new(LinkedList::new()));
    let mut handles = Vec::new();




    let id = 0b10000000;
    for num in 0..11 {
        let list = Arc::clone(&user_list);
        let handle = thread::spawn(move ||{
            thread::sleep(Duration::from_millis(1));
            let mut lst = list.lock().unwrap();
            lst.add(id + num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let mut lst = user_list.lock().unwrap();
    println!("Players in queue: {}", lst.len());
    while lst.len() > 0 {
        println!("{}", lst.pop());
    }
}



struct StateNode {
    state: u8,
    next: Option<Box<StateNode>>
}


impl StateNode{
    fn new(state: u8) -> StateNode{
        return StateNode {state: state, next: None};
    }
    fn chain_node(state: u8, next: StateNode) -> StateNode{
        return StateNode {state: state, next: Some(Box::new(next))};
    }

    fn getValue(&self) -> u8 {
        return self.state;
    }
}

struct LinkedList {
    head: Option<Box<StateNode>>
}

impl LinkedList{
    fn new() -> LinkedList {
        return LinkedList {head: None};
    }

    fn add(&mut self, state: u8){
        let new_node = StateNode {state: state, next: None};
        let mut curr = &mut self.head;
        while let Some(node) = curr{
            curr = &mut node.next;
        }
        *curr = Some(Box::new(new_node));
    }
    fn pop(&mut self) -> u8{
        let node = self.head.take().unwrap();
        let result = node.getValue();
        self.head = node.next;
        result
    }

    fn len(&self) -> usize{
        let mut length = 0;
        let mut curr = &self.head;
        while let Some(node) = curr{
            curr = &node.next;
            length+=1;
        }
        length
    }
}

impl Drop for LinkedList {
    fn drop(&mut self){
        self.head = None;
        println!("Dropped");
    }
}