struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;  // Requires A and B
    fn first(&self) -> i32;  // Doesnt require A or B
    fn last(&self) -> i32;  // Doesnt require A or B
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 { self. 0}

    fn last(&self) -> i32 { self.1 }
}

fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
    container.last() - container.first()
}

trait AssociatedContains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl AssociatedContains for Container {
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

fn associated_difference<C: AssociatedContains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        <Container as Contains<i32, i32>>::contains(&container, &number_1, &number_2));
    // println!("First number: {}", Container::first(&container));
    // println!("Last number: {}", Container::last(&container));

    // println!("The difference is: {}", difference(&container));
    println!("The difference is: {}", associated_difference(&container));
}