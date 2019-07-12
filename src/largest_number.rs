pub fn largest() {

    println!("Enter number of elements:");
    let number_of_elements : i32 = read!();
    let mut num_vector = Vec::new();

    let mut largest_num = 0;

    for i in 0..number_of_elements {
        println!("Enter element {}:", i + 1);
        let element : i32 = read!();
        num_vector.push(element);

        if element >= largest_num {
            largest_num = element;
        }
    }

    println!("Largest number is: {}", largest_num);
}

