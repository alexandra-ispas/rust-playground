#[derive(Debug)]
enum NumberType {
    Home,
    Work,
    Cell,
}

#[derive(Debug)]
struct Contact {
    name: String,
    phone_number: String,
    number_type: NumberType,
}

impl Contact {
    fn new(name: &str, phone_number: &str, number_type: NumberType) -> Self {
        Self {
            name: name.to_string(),
            phone_number: phone_number.to_string(),
            number_type,
        }
    }

    fn display(&self) {
        println!(
            "Name: {}, Phone Number: {}, Type: {:?}",
            self.name, self.phone_number, self.number_type
        );
    }
}

fn main() {
    let contacts = vec![
        Contact::new("Jane & John Doe", "555-123-4567", NumberType::Home),
        Contact::new("Joseph Bloggs", "555-111-2222", NumberType::Work),
        Contact::new("Rowena Stevenson", "555-111-3333", NumberType::Cell),
        Contact::new("Marcus Mills", "555-111-4444", NumberType::Cell),
        Contact::new("Orson Richards", "555-111-5555", NumberType::Cell),
    ];

    for contact in contacts {
        contact.display();
    }
}
