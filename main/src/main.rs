use libscan::*;


/*
 *  read!( i as u32)
 *  read_vec! ( i as u32)
 *  linked_list.push(3);
    linked_list.print();
    linked_list.insert_at_index(2, 10);
    linked_list.delete_at_index(0);
 *
 *
 *
 */

fn main() {
    let mut i:u32;
    read!( i as u32);
    println!("i is {}",i);
    let mut i:Vec<String>;
    read_vec!( i as String);
    println!(" i[0] = {:?}",i[0]);
      let mut linked_list: LinkedList<i32> = LinkedList::new();

    // Add elements to the linked list
    linked_list.push(3);
    linked_list.push(2);
    linked_list.push(1);

    // Print the elements of the linked list
    linked_list.print();

    // Insert element at index 1
    linked_list.insert_at_index(1, 4);

    // Print the updated linked list
    println!("After insertion:");
    linked_list.print();

    // Delete element at index 2
    linked_list.delete_at_index(2);

    // Print the linked list after deletion
    println!("After deletion:");
    linked_list.print();
}
