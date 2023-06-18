pub fn run() {
    let city_a = City::new("City A", 100_000);
    let city_b = City::new("City B", 250_000);
    let city_c = City::new("City C", 40_000);

    let cities = vec![city_a, city_b, city_c];

    let country = Country::from(cities);

    country.print_cities();
}

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}", city.name, city.population);
        }
    }
}
