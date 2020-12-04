fn count_complete_passwords(input: Vec<String>) -> usize {
    let passports: Vec<Passport> = parse_passports(input);
    passports.iter().filter(|passport| passport.has_necessary_fields()).count()
}

fn count_valid_passwords(input: Vec<String>) -> usize {
    let passports: Vec<Passport> = parse_passports(input);
    passports.iter().filter(|passport| passport.is_valid()).count()
}

fn parse_passports(input: Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = vec![Passport::default()];
    for line in input {
        let parts = line.split(" ");
        for part in parts {
            let mut key_value = part.split(":");
            let key = key_value.next().unwrap();
            let value = key_value.next();
            if let Some(value) = value {
                let last_entry = passports.last_mut().unwrap();
                last_entry.add_by_key(key, value)
            } else {
                passports.push(Passport::default());
            }
        }
    }
    passports
}

#[derive(Default)]
struct Passport {
     birth_year: Option<String>,
     issue_year: Option<String>,
     expiration_year: Option<String>,
     height: Option<String>,
     hair_color: Option<String>,
     eye_color: Option<String>,
     passport_id: Option<String>,
     country_id: Option<String>
}

impl Passport {
    fn add_by_key(&mut self, key: &str, value: &str) -> () {
        match key {
            "byr" => self.birth_year = Some(value.to_owned()),
            "iyr" => self.issue_year = Some(value.to_owned()),
            "eyr" => self.expiration_year = Some(value.to_owned()),
            "hgt" => self.height = Some(value.to_owned()),
            "hcl" => self.hair_color = Some(value.to_owned()),
            "ecl" => self.eye_color = Some(value.to_owned()),
            "pid" => self.passport_id = Some(value.to_owned()),
            "cid" => self.country_id = Some(value.to_owned()),
            _ => println!("What the heck is {}??", key)
        }
    }

    fn has_necessary_fields(&self) -> bool {
        self.birth_year.is_some() &&
            self.issue_year.is_some() &&
            self.expiration_year.is_some() &&
            self.hair_color.is_some() &&
            self.height.is_some() &&
            self.eye_color.is_some() &&
            self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        if !self.has_necessary_fields() {
            return false;
        }

        let birth_year = self.birth_year.as_ref().unwrap().parse::<usize>().unwrap();
        if birth_year < 1920 || birth_year > 2002 {
            return false;
        }

        let issue_year = self.issue_year.as_ref().unwrap().parse::<usize>().unwrap();
        if issue_year < 2010 || issue_year > 2020 {
            return false;
        }

        let expiration_year = self.expiration_year.as_ref().unwrap().parse::<usize>().unwrap();
        if expiration_year < 2020 || expiration_year > 2030 {
            return false;
        }

        if let Some(value) = self.height.as_ref().unwrap().strip_suffix("in") {
            let height = value.parse::<usize>().unwrap();
            if height < 59 || height > 76 { return false;}
        } else if let Some(value) = self.height.as_ref().unwrap().strip_suffix("cm") {
            let height = value.parse::<usize>().unwrap();
            if height < 150 || height > 193 { return false;}
        } else {
            return false;
        }

        // hair color
        if let Some(value) = self.hair_color.as_ref().unwrap().strip_prefix("#") {
            if i64::from_str_radix(value, 16).is_err() { return false;}
        } else { return false }

        // eye color
        let valid_eye_color = match &self.eye_color.as_ref().unwrap()[..] {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false
        };
        if !valid_eye_color {
            return false;
        }

        // passport ID
        if self.passport_id.as_ref().unwrap().len() != 9 {
            return false;
        }
        if self.passport_id.as_ref().unwrap().parse::<usize>().is_err() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::read_file::read_file_to_vec;
    use crate::puzzle_04::{count_complete_passwords, count_valid_passwords};

    #[test]
    fn solve_day_04_1() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_04.txt");

        println!(
            "There are {} complete passports",
            count_complete_passwords(input)
        );
    }

    #[test]
    fn solve_day_04_2() {
        let input: Vec<String> = read_file_to_vec("input/puzzle_04.txt");

        println!(
            "There are {} valid passports",
            count_valid_passwords(input)
        );
    }
}
