// 2.0 A Bad Stack!

use std::mem;

pub struct List{
    head: Link,
}


enum Link{
    Empty,
    // This<T> is used to specify a generic type
    More(Box<Node>),
}

struct Node{
    elem: i32,
    next: Link,
}

impl List{
    // Self referes to impl Type
    pub fn new() -> Self{
        // The last expression of a function is implicitly returned
        List { head: Link::Empty}
    }


    pub fn push(&mut self, elem:i32){
        let new_node = Box::new(Node{
            elem:elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    // Option<T> indicates that it can be None or Some,
    // Some(let T) encapsulating the value of var datatype T
    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            // This node is an alias for the type inside of More()
            Link::More(node)=>{
            self.head = node.next;
            Some(node.elem)
            }
        }
        // unimplemented!() is a macro, macros are defined with the ! sign
        // It panics in a controlled way
        // unimplemented!()
        // result
    }
}

// Drop is an interface that defines what happens when a value
// goes out of scope, we will implement it to list
impl Drop for List{
    fn drop(&mut self){
        // In rust vars are inmutable by default.
        // So we use mut.
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // Walrus operator overhere, we declare the var mut boxed_node
        // that will be the cur_link if the cur_link is a Link::More
        while let Link::More(mut boxed_node) = cur_link{
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

// This line means that the test will only be compiled when we are running tests.
#[cfg(test)]
mod test{
    use super::List;
    #[test]
    fn basics(){
        let mut list = List::new();

        // Check Empty
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}
