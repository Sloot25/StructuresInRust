struct Node<T>{
    elem: T,
    next: Option<Node<T>>,
    previous: Option<Node<T>>,
}

pub struct List<T>{
    head: Node<T>,
    tail: Node<T>,
    size: i32, 
}

impl List<T>{
    pub fn new() -> Self {
	List { head:}
    }
}
