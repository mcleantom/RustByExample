fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // If x is an Option then evaluating ? will return the underlying value
    // otherwise it will terminate the function and return None
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        // using ? makes this much more readable
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 123456789,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}