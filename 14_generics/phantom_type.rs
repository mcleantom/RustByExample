use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

// Having PhantomData<Unit> allows us to hold onto the enum Inch or Mm at compile time
// but have no knowledge/use of it at run time. I.e. we can use it as a compile time check
// that the units are correct but there will be no memory increase or speed loss at runtime
/*
The alternative to this would be to do something like

enum Unit {
    Inch,
    Mm,
}

struct Length {
    value: f64,
    unit: Unit,
}

However this would mean that we are now holding onto the unit at runtime and so
there would be a memory increase as we are now holding onto an extra value.

*/
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// implement new
impl<Unit> Length<Unit> {
    fn new(value: f64) -> Self {
        Length(value, PhantomData)
    }
}

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot = Length::<Inch>::new(12.0);
    let one_meter = Length::<Mm>::new(1000.0);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("{:?}", two_feet.0);
    println!("{:?}", two_meters.0);

    let one_feeter = one_foot + one_meter;
}