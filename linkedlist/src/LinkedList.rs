struct Node<T>{
    elem: T,
    next: Option<Box<Node<T>>>,
    previous: Option<Box<Node<T>>>,
}

pub struct List<T>{
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    size: i32, 
}

impl List<T>{
    pub fn new() -> Self {
	List {
	    head: None,
	    tail: None,
	    size: 0,
	}
    }
    pub fn push(&mut self, elem: T){
	let mut newNode = Node{
	    elem: elem,
	    next: None,
	    previous: None,
	};
	self.size = self.size + 1;
	if head == None {
	    self.head = newNode;
	    self.tail = newNode; 
	} else {
	    newNode.previous = self.tail;
	    self.tail.next = newNode;
	    self.tail = newNode;
	}
    }

    pub fn pop(&mut self) -> Option<T> {
	let result = self.tail;
	if self.tail == self.head {
	    self.tail = None;
	    self.head = None;
	} else{
	    self.tail = self.previous;
	    self.tail.next = None; 
	}
	self.size = self.size - 1;
	result
    }

    pub fn getSize(&self) -> i32 {
	self.size
    }

    pub fn isEmpty(&self) -> bool {
	tail == None
    }

    pub fn addEnd(&mut self, elem: T) {
	self.push(elem);
    }

    pub fn addStart(&mut self, elem: T){
	let mut newNode = Node {
	    elem: elem,
	    previous: None,
	    next: None,
	}
	self.size = self.size + 1;
	if self.head == None {
	    self.head = newNode;
	    self.tail = newNode;
	}else {
	    newNode.next = self.head;
	    self.head.previous = newNode;
	    self.head = newNode; 
	}
    }

    pub fn insert(&mut self, index: i32, elem: T) {
	if index < 1 {
	    self.addStart(elem);
	} else if index > self.size-1 {
	    self.addEnd(elem);
	} else {
	    self.size = self.size + 1;
	    let mut m = self.recorre(index);
	    let mut n = Node{
		elem: elem,
		previous: m.previous,
		next: m,
	    }
	    m.previous.next = n;
	    m.previous = n; 
	}
    }

    fn recorre(&mut self, index: i32) -> Option<Node<T>> {
	let actual = self.head;
	let i = 0;
	while actual != None {
	    if i == index {
		actual
	    }
	    actual = actual.next;
	    i = i + 1;
	}
	actual
    }
}
