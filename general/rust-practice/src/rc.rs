#![allow(dead_code)]

use std::rc::Rc;

#[derive(Debug)]
struct Employee {
    name: Rc<String>,
    id: String,
    company: Rc<String>,
    city: Rc<Box<String>>
}

impl Employee {
    fn new (name: Rc<String>, id: String, company: Rc<String>, city: Rc<Box<String>>) -> Employee {
        Employee {
            name: name,
            id: id,
            company: company,
            city: city
        }
    }
}

mod test {
    use super::Employee;
    use std::rc::Rc;

    #[test]
    fn reference_count_test () {
        let name1 = Rc::new("Venkat".to_string());
        let id1 = "200963".to_string();
        let company = Rc::new("ABC Inc".to_string());
        let box_string_city: Box<String> = Box::new("Bangalore".to_string());
        let city = Rc::new(box_string_city);

        let name2 = Rc::new("Vadhri".to_string());
        let id2 = "200964".to_string();

        let e1 = Employee::new(name1.clone(), id1, company.clone(), city.clone());
        let e2 = Employee::new(name2.clone(), id2, company.clone(), city.clone());

        assert_eq!(Rc::strong_count(&company), 3);
    }
}
