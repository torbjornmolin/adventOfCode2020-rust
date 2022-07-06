#[derive(Debug, Clone)]
pub struct Rule<'a> {
    pub name: &'a str,
    bound1: Bound,
    bound2: Bound,
    index: Option<usize>
}
#[derive(Debug, Clone)]
pub struct Bound {
    low: usize,
    high: usize
}
impl Bound {
    pub fn from_string(s: &str) -> Bound {
        let parts: Vec<&str> = s.split('-').collect();
        let low: usize = parts[0].trim().parse().unwrap();
        let high: usize = parts[1].trim().parse().unwrap();

        Bound {
            low,
            high
        }
    }
}

impl Rule<'_> {
    pub fn from_string(s: &str) -> Rule{
        let name_and_bounds: Vec<&str> = s.split(':').collect();
        let name = name_and_bounds[0];

        let bounds: Vec<&str> = name_and_bounds[1].split("or").collect();

        let bound1 = Bound::from_string(bounds[0]);
        let bound2 = Bound::from_string(bounds[1]);

        Rule {
            name,
            bound1,
            bound2,
            index: Option::None
        }
    }
    pub fn matches(&self, num: usize) -> bool {
        if num >= self.bound1.low && num <= self.bound1.high {
            return true;
        }
        if num >= self.bound2.low && num <= self.bound2.high {
            return true;
        }

        false
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    pub fn set_index(&mut self, index: usize) {
        if self.index.is_some() {
            panic!("Cant set associated index twice!");
        }
        self.index = Option::Some(index);
    }
    pub fn get_index(&self) -> Option<usize> {
        self.index
    }
}