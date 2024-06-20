struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = "Batu Khasikov".to_string();

    let kalmykia = Country {
        population,
	capital,
	leader_name,
    };

    println!("{} {} {}", kalmykia.population, kalmykia.capital, kalmykia.leader_name);
}
