pub fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);

    println!("v1 is {:?}", v1);
    println!("v2 is {:?}", v2);
    println!("v3 is {:?}", v3);
    read_elements();
}

fn read_elements() {
    let v = vec![1, 2, 3, 4 , 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get returns Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![1, 2, 3,];
    for i in &mut v2 {
        // To change the value that the mutable reference refers to,
        // we have to use the dereference operator * to get the value in i
        *i += 10;
    }
    println!("v2 after mutation {:?}", v2);
}