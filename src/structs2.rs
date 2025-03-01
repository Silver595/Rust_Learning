struct Football {
    footballer_first_name: String,
    footballer_last_name: String,
    footballer_jersey_no: String,
}

impl Football {
    fn football_team(first_name: &str, last_name: &str, jersey_no: &str) -> Football {
        Football {
            footballer_first_name: first_name.to_string(),
            footballer_last_name: last_name.to_string(),
            footballer_jersey_no: jersey_no.to_string(),
        }
    }

    fn football_info(&self) -> String {
        format!(
            "{}, {}, {}",
            self.footballer_first_name, self.footballer_last_name, self.footballer_jersey_no
        )
   }

   fn change_first_name(&mut self,first_name:&str){
	self.footballer_first_name = first_name.to_string()
}
}

fn main() {
    let mut football1 = Football::football_team("Diego", "Maradona", "10");

    println!(
        "Football Player Info: first name: {}, last name: {}, jersey no: {}",
        football1.footballer_first_name, football1.footballer_last_name, football1.footballer_jersey_no
    );

    println!("Football Player Info: {}", football1.football_info());

   football1.change_first_name("lionel");

   println!("new name: {}",football1.football_info());
}

