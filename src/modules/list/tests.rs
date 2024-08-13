use super::list::{self, List};

#[test]
fn add_operation() {
    let mut list_of_int: List<i32> = list::List::new();

    list_of_int.add(2);
    list_of_int.add(-1);

    assert_eq!(2, list_of_int.size());
    assert_eq!(100, list_of_int.capacity());
    assert_eq!(-1, *list_of_int.get(1).unwrap())
}

#[test]
fn delete_operation() {
    let mut list_of_int: List<i32> = list::List::new();

    list_of_int.add(2);
    list_of_int.add(-1);
    
    let number = list_of_int.delete_last();

    assert_eq!(1, list_of_int.size());
    assert_eq!(100, list_of_int.capacity());
    assert_eq!(-1, number.unwrap());
}

#[test]
fn insert_at_operation() {
    let mut list_of_int: List<i32> = list::List::new();

    list_of_int.add(2);
    list_of_int.add(-1);
    list_of_int.add(-3);
    list_of_int.add(-13);
    list_of_int.add(-14);
    
    list_of_int.insert_at(3, 1);
    // println!("{}", list_of_int);
}