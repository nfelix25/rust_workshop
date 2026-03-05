const TOWN_SIZE: u32 = 1000;
const CITY_SIZE: u32 = 10_000;
const METROPOLIS_SIZE: u32 = 1_000_000;

#[repr(u32)] // The `repr` attribute allows us to specify how the enum should be represented in memory.
enum CitySize {
    Town = TOWN_SIZE,
    City = CITY_SIZE,
    Metropolis = METROPOLIS_SIZE,
    Size(u32), // This variant allows us to create a city of any size, not just the predefined ones.
}

impl CitySize {
    fn value(&self) -> u64 {
        match self {
            CitySize::Town => TOWN_SIZE as u64,
            CitySize::City => CITY_SIZE as u64,
            CitySize::Metropolis => METROPOLIS_SIZE as u64,
            CitySize::Size(size) => *size as u64,
        }
    }
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn get_descriptor_from_pop(pop: u64) -> &'static str {
        //'static str means that the string slice has a static lifetime, which means it will live for the entire duration of the program (eg on the heap not the stack)
        if pop < TOWN_SIZE as u64 {
            "village"
        } else if pop < CITY_SIZE as u64 {
            "town"
        } else if pop < METROPOLIS_SIZE as u64 {
            "city"
        } else {
            "metropolis"
        }
    }

    fn get_descriptor_from_self(&self) -> &'static str {
        City::get_descriptor_from_pop(self.residents)
    }

    fn describe(residents: u64, is_coastal: bool) -> String {
        format!(
            "A *{}* *{}* of approxiiamtely {} residents",
            if is_coastal { "Coastal" } else { "Non-Coastal" },
            City::get_descriptor_from_pop(residents),
            residents
        )
    }

    fn describe_self(&self) -> String {
        City::describe(self.residents, self.is_coastal)
    }

    fn new(city_size: CitySize, is_coastal: bool) -> Self {
        let residents = city_size.value();
        let (description, residents) = match city_size {
            CitySize::Town => (City::describe(residents, is_coastal), residents),

            CitySize::City => (City::describe(residents, is_coastal), residents),

            CitySize::Metropolis => (City::describe(residents, is_coastal), residents),

            CitySize::Size(_size) => (City::describe(residents, is_coastal), residents),
        };

        City {
            residents,
            description,
            is_coastal,
        }
    }
}

fn get_metro_size(city: City) -> Result<u64, String> {
    if u64::from(METROPOLIS_SIZE) == city.residents {
        Ok(city.residents)
    } else {
        Err(format!(
            "{} is not a metropolis, it is a {}",
            city.description,
            city.get_descriptor_from_self()
        ))
    }
}

fn main() {
    let rustville: City = City::new(CitySize::Metropolis, false);

    println!("{}", rustville.description);

    if rustville.residents >= CITY_SIZE as u64 {
        println!("Wow, that's a big city!");
    }

    if let Ok(metro_size) = get_metro_size(rustville) {
        println!("The metro has {} residents!\n", metro_size);
    } else {
        println!("Ain't no metro, yo.")
    }

    let rustville2: City = City::new(CitySize::Town, true);

    println!("{}", rustville2.describe_self());

    if rustville2.residents >= CITY_SIZE as u64 {
        println!("Wow, that's a big city!");
    } else {
        println!("Itty, bitty city committee!");
    }

    match get_metro_size(rustville2) {
        Ok(metro_size) => {
            println!("The metro has {} residents!", metro_size);
        }
        Err(error) => {
            println!("Ain't no metro, yo. Error: {}", error);
        }
    }

    let rustville3: City = City::new(CitySize::Size(10), false);

    println!("\n{}", rustville3.describe_self());

    if rustville3.residents >= CITY_SIZE as u64 {
        println!("Wow, that's a big city!");
    } else {
        println!("Itty, bitty city committee!");
    }

    match get_metro_size(rustville3) {
        Ok(metro_size) => {
            println!("The metro has {} residents!", metro_size);
        }
        Err(error) => {
            println!("Ain't no metro, yo. Error: {}", error);
        }
    }

    let rustville4: City = City::new(CitySize::City, false);

    println!("\n{}", rustville4.describe_self());

    if rustville4.residents >= CITY_SIZE as u64 {
        println!("Wow, that's a big city!");
    } else {
        println!("Itty, bitty city committee!");
    }

    match get_metro_size(rustville4) {
        Ok(metro_size) => {
            println!("The metro has {} residents!", metro_size);
        }
        Err(error) => {
            println!("Ain't no metro, yo. Error: {}", error);
        }
    }
}
