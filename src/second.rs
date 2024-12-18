// 3.0 An Ok Stack

pub struct List<T>{
    head: Link<T>,
}

/*
enum Link{
    Empty,
    More(Box<Node>),
}
*/
type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem: T,
    next: Link<T>,
}
//First <T> anounces generic type
//Second <T> says, i implement this to this exact type, in this case all of them
impl<T> List<T>{
    pub fn new() -> Self{
        List {head: None}
    }
    
    pub fn peek(&self) -> Option<&T>{
        // as_ref hace que pasemos algo como una referencia.
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T>{
        // o as_mut hace que pasemos algo como referencia mutable.
        self.head.as_mut().map(|node|{
            // this is saying return an adress to node.elem.
            &mut node.elem
        })
    }

    pub fn push(&mut self, elem:T){
        let new_node = Box::new(Node{
            elem:elem,
            // next: mem::replace(&mut self.head, None),
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T>{
        /* match self.head.take(){
            None => None,
            More(node)=>{
                self.head = node.next;
                Some(node.elem)
            }
        }
        */
        // We use map to which we have to pass a closure.
        // This shit says, if self.head.take() is NOT null, 
        // self.head.take=node and we run the function. 
        self.head.take().map(|node| {
            self.head=node.next;
            node.elem
        })
    }
}


impl<T> Drop for List<T>{
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link{
            cur_link = boxed_node.next.take();
        }
    }
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
}


#[cfg(test)]
mod test{
    use super::List;
    #[test]
    fn basics(){
        let mut list = List::new();

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
