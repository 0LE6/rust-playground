// Characteristics of OOP

fn main() {
    let mut coll = AveragedCollection {
        list: vec![],
        average: 0.0,
    }; 
    
    coll.print_collection();

    coll.add(69);

    coll.print_collection();

    coll.add(100);

    coll.print_collection();

    coll.remove();

    coll.print_collection();
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average        
    }

    pub fn print_collection(&self) {
        println!(
            "{:?} \nAverage: {:?}\n---------",
            self.list,
            self.average,
        );
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average 
            = total as f64 / self.list.len() as f64;
    }

}
