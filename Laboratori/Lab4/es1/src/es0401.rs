pub mod List1 {
    use std::mem;
    use std::ptr::null;

    pub enum ListLink<T> {
        Cons(T, Box<ListLink<T>>),
        Nil,
    }

    pub struct List<T> {
        pub head: ListLink<T>,
    }

    impl<T: Clone> List<T> {
        pub fn new() -> Self {

            let head = ListLink::Nil;

            List { head }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // why? look at mem::replace for solving it
        pub fn push(&mut self, elem: T) {

            let new_node = ListLink::Cons(elem, Box::new(std::mem::replace(&mut self.head, ListLink::Nil)));

            self.head = new_node;
        }

        pub(crate) fn pop(&mut self) -> Option<T> {

            let old_node = mem::replace(&mut self.head, ListLink::Nil);

            let old_value: T;
            let old_next: ListLink<T>;

            match old_node {
                ListLink::Cons(elem, next) => {
                    old_value = elem;
                    old_next = *next;
                    self.head = old_next;
                    Some(old_value)
                },
                ListLink::Nil => return None
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {

            match &self.head {
                ListLink::Cons(elem, next) => Some(elem),
                ListLink::Nil => None
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        //fn iter(&self) -> ListIter<T> {
        //    unimplemented!()
        //}

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list = List { head: ListLink::Nil };
            let mut current = &self.head;
            for _ in 0..n {
                if let ListLink::Cons(value, next) = current {
                    new_list.push(value.clone());
                    current = next;
                } else {
                    break;
                }
            }
            new_list
        }
    }


    //struct ListIter {
    //    // implement the iterator trait for ListIter
    //}
    //
    //impl Iterator for ListIter {
    //    //type Item = ...
    //
    //    fn next(&mut self) -> Option<Self::Item> {
    //        unimplemented!()
    //    }
    //}

    // something that may be useful for the iterator implementation:
    // let a = Some(T);
    // let b = &a;
    // match b { Some(i) => ... } // here i is a reference to T

}


pub mod List2 {
    use std::cell::RefCell;
    use std::mem;
    use std::rc::Rc;


    pub struct Node<T> {
        pub elem: T,
        pub next: NodeLink<T>,
        pub prev: NodeLink<T>
    }

    type NodeLink<T> = Option<Rc<RefCell<Node<T>>>>;

    pub struct List<T> {
        pub head: NodeLink<T>,
        pub tail: NodeLink<T>
    }



    // for this implementattion, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        pub fn new() -> Self {

            let head = None;
            let tail = None;

            List { head, tail }
        }

        // insert a new element at the beginning of the list
        pub fn push_head(&mut self, elem: T) {

            // creo il nuovo nodo
            let new_node = Node { elem, next: self.head.clone(), prev: None };

            let new_nodelink = Some(Rc::new(RefCell::new(new_node)));   // creo il nodeLink

            match &self.head {
                Some(old_head) => old_head.borrow_mut().prev = new_nodelink.clone(),
                None => (),
            }

            self.head = new_nodelink.clone();

            if self.tail.is_none() {
                self.tail = new_nodelink.clone();
            }
        }

        pub fn push_tail(&mut self, elem: T) {

            let new_node = Node { elem, next: None, prev: self.tail.clone() };
            let new_nodelink = Some(Rc::new(RefCell::new(new_node)));

            match &self.tail {
                Some(old_tail) => old_tail.borrow_mut().next = new_nodelink.clone(),
                None => (),
            }

            self.tail = new_nodelink.clone();

            if self.head.is_none() {
                self.head = new_nodelink.clone();
            }
        }

        pub fn pop_head(&mut self) -> Option<T> {

            match self.head.take() {
                Some(head) => {
                    match &head.borrow().next {
                        Some(next) => next.borrow_mut().prev = None,
                        None => ()
                    }

                    self.head = head.borrow_mut().next.take();

                    return Some(head.into_inner().elem)
                },
                None => return None
            }

        }
        /*
        pub(crate) fn pop(&mut self) -> Option<T> {

            let old_node = mem::replace(&mut self.head, ListLink::Nil);

            let old_value: T;
            let old_next: ListLink<T>;

            match old_node {
                ListLink::Cons(elem, next) => {
                    old_value = elem;
                    old_next = *next;
                    self.head = old_next;
                    Some(old_value)
                },
                ListLink::Nil => return None
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {

            match &self.head {
                ListLink::Cons(elem, next) => Some(elem),
                ListLink::Nil => None
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        //fn iter(&self) -> ListIter<T> {
        //    unimplemented!()
        //}

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> crate::es0401::List1::List<T> {
            let mut new_list = crate::es0401::List1::List { head: ListLink::Nil };
            let mut current = &self.head;
            for _ in 0..n {
                if let ListLink::Cons(value, next) = current {
                    new_list.push(value.clone());
                    current = next;
                } else {
                    break;
                }
            }
            new_list
        } */
    }
}

// *****
// double linked list suggestion: use Rc, since we need more than one reference to the same node
// for mutating the list and changing the next and prev fields we also need to be able to mutate the node, therefore we can use RefCell

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// to take a value from a Rc (useful when popping a value from the list): usually it is not possible since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  youu can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore: 
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None

//  type NodeLink = Option<Rc<RefCell<DNode>>>; // we define a type alias for better readibility
// Example
//  type NodeBackLink = ... 

// struct DNode {
    // v: i32,
    // prev: NodeBackLink // here we can't put NodeLink to avoid a cycle reference, what do we use?
    // next: NodeLink 
// }

// struct DList {
    // head: NodeLink,
    // tail: NodeLink
// }