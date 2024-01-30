#[derive(Debug)]
struct Car {
    model: String,
    year: i32,
}

fn upgrade_car(mut car: Car) -> Car {
    car.year += 1;

    car
}

pub fn main_hard() {
    let my_car = Car {
        model: String::from("Sedan"),
        year: 2022,
    };

    let upgraded_car = upgrade_car(my_car);

    println!("Upgraded car: {:?}", upgraded_car);
}
