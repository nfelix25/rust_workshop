fn main() {
    let city_name: &str = "Rustville";

    println!("The city of {}:\n", city_name);

    print_population(1_234_567, 123_456, 102_345);
}

fn print_population(adults: u64, kids: u32, buildings: u32) {
    let population: u32 = adults as u32 + kids;

    let buildings_per_person: f32 = buildings as f32 / population as f32;

    println!("Population: {}", population);
    println!("    Adults: {}", adults);
    println!("      Kids: {}", kids);
    println!(" Buildings: {}", buildings);
    println!(" Buildings per person: {}", buildings_per_person);

    let output: &str = if buildings_per_person >= 1.0 {
        "Everyone can have their own building!!!"
    } else {
        "Get comfy you fuckin' plebs!"
    };

    println!("{}", output);
}
