fn main() {
    let mut city_names: Vec<&str> = Vec::with_capacity(2);
    city_names.push("Pythonia");
    city_names.push("Javasburg");
    println!("Capacity after adding 2 cities: {}", city_names.capacity()); // Will be 2
    println!(
        "\nAddress of vector data before resize: {:p}\n",
        city_names.as_ptr()
    );
    city_names.push("C by the sea");
    println!(
        "RESIZE: Capacity after adding 3rd city: {}",
        city_names.capacity()
    ); // Will be 4 (doubled from 2)
    println!(
        "\nAddress of vector data after resize: {:p}\n",
        city_names.as_ptr()
    );

    println!("Capacity before shrink: {}", city_names.capacity());
    city_names.shrink_to_fit();
    println!("Capacity after shrink: {}", city_names.capacity()); // Will be 3
    println!(
        "\nAddress of vector data after shrink (same as after resize): {:p}\n",
        city_names.as_ptr()
    );

    if let Some(last_city) = city_names.pop() {
        if last_city.starts_with("R") {
            println!("{last_city} starts with an R!");
        } else {
            println!("{last_city} does not start with an R!");
        }
        city_names.push(last_city);
    }

    let last_city = match city_names.pop() {
        Some(possible_name) => possible_name,
        None => "", // Theoretically would default to empty string if the vector was empty --- Properly anrrows last_city type to &str
    };

    if last_city.ends_with("a") {
        println!("{} ends with an a!", last_city);
    } else {
        println!("{} does not end with an a!", last_city);
    }

    println!(
        "Cities: (printed with :? --- debug flag) {:?}\n",
        city_names
    );
    println!("Cities: (printed with iter() loop)");

    for name in city_names.iter() {
        println!("{}", name);
    }
}
