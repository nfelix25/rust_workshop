struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn describe(&self) -> String {
        describe_city((self.residents, self.is_coastal))
    }

    fn new(residents: u64, is_coastal: bool) -> Self {
        new_city(residents, is_coastal)
    }
}

fn describe_city(descriptors: (u64, bool)) -> String {
    format!(
        "A *{}* city of approximately {} residents.",
        if descriptors.1 {
            "coastal"
        } else {
            "non-coastal"
        },
        descriptors.0
    )
}

struct CityArgs {
    residents: u64,
    is_coastal: bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    let args: CityArgs = CityArgs {
        residents,
        is_coastal,
    };

    let CityArgs {
        is_coastal: is_coastal_arg,
        ..
    } = args;
    let CityArgs {
        residents: residents_arg,
        ..
    } = args;

    if is_coastal {
        City {
            description: describe_city((residents_arg, is_coastal_arg)),
            residents,
            is_coastal,
        }
    } else {
        City {
            description: describe_city((residents_arg, is_coastal_arg)),
            residents: residents_arg,
            is_coastal: is_coastal_arg,
        }
    }
}

fn main() {
    let residents_args: [u64; 2] = [1_234_567, 2_345_678];
    let coastal_args: [bool; 2] = [true, false];

    let [_, r_args_1] = residents_args;
    let [c_args_0, _] = coastal_args;

    let rustville: City = City::new(r_args_1, c_args_0);
    let rustville2: City = City::new(residents_args[0], coastal_args[1]);

    println!(
        "City 1 is such a city, it's like: {}.\nPopulation: {}\nCoastal: {}\n\n",
        rustville.description, rustville.residents, rustville.is_coastal
    );
    println!(
        "City 2 is such a city, it's like: {}.\nPopulation: {}\nCoastal: {}\n\n",
        rustville2.describe(),
        rustville2.residents,
        rustville2.is_coastal
    );
}
