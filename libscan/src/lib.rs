// Define a Node struct
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Define a LinkedList struct to encapsulate the linked list operations
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}


impl<T: std::fmt::Display> LinkedList<T> { //impl<T> LinkedList<T> 
    // Create a new empty linked list
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert_at_index(&mut self, index: usize, data: T) {
        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                // Index out of bounds, insert at the end
                self.push(data);
                return;
            }
        }

        let new_node = Node {
            data,
            next: current.take(),
        };
        *current = Some(Box::new(new_node));
    }

    // Delete the element at a specific index in the linked list
    pub fn delete_at_index(&mut self, index: usize) {
        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                // Index out of bounds, do nothing
                return;
            }
        }

        *current = current.take().and_then(|node| node.next);
    }

    // Add a new element to the front of the linked list
    pub fn push(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    // Print the elements of the linked list
    pub fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}", node.data);
            current = &node.next;
        }
    }
}

    

/* Variable stdin */

#[macro_export]
macro_rules! read{
    ($out:ident as $type:ty)=>{
        let mut line:String= String::new();
        std::io::stdin().read_line(&mut line);
        $out = line.trim().parse::<$type>().unwrap();
    };
}

/* Vector array stdin */

#[macro_export]
macro_rules! read_vec{
    ($out:ident as $type:ty)=>{
        let mut line:String= String::new();
        std::io::stdin().read_line(&mut line);
        $out = line.trim().split_whitespace().map(|x| x.parse::<$type>().unwrap()).collect::<Vec<$type>>()
    };
}


