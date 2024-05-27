
pub struct Arguments {
    gotarguments: Vec<String>,
    allargs: Vec<String>,
    pub arguments: Vec<String>,
}

impl Arguments {
    pub fn new(args: Vec<String>) -> Self {
        Arguments {
            gotarguments: args,
            allargs: Vec::new(),
            arguments: Vec::new(),
        }
    }

    pub fn add(&mut self, argument: &str) {
        self.allargs.push(argument.to_string());
    }
    
    pub fn analyze(&mut self) {
        for x in &self.allargs {
            if self.gotarguments.contains(x) {
                self.arguments.push(x.clone());
            }
        }
    }

    pub fn fetch(&self, ind: usize) -> String {
        // println!("{}",ind);
        self.gotarguments[ind-1].clone()
    }

    pub fn gotlen(&self) -> usize {
        self.gotarguments.len()
    }
}