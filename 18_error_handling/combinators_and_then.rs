#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday
}

fn have_ingreedients(food: Food) -> Option<Food> {
    // we dont have the ingreedients to make sushi
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food)
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingreedients(food)
    }
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingreedients)
}

fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingreedients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("on {:?} we ate {:?}", day, food),
        None => println!("We didnt eat on {:?}", day)
    }
}

fn main() {
    eat(Food::CordonBleu, Day::Monday);
    eat(Food::Steak, Day::Tuesday);
    eat(Food::Sushi, Day::Wednesday);
}