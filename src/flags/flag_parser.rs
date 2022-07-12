use std::collections::HashMap;
use crate::flags::flag::Flag;

pub struct FlagParser {
    /// All flags that are provided by the cli
    flags: Vec<Flag>
}

impl FlagParser {

    /// Creates a new instance of the flag parser
    pub fn new(flags: Vec<Flag>) -> Self {
        FlagParser {flags}
    }

    /// Parses all provided flags into a hashMap
    /// If one of the provided flags is unknown the function will
    /// return None and the cli will throw an error
    pub fn parse_flags(&mut self) -> Option<HashMap<String, Option<String>>> {

        let mut args = std::env::args();
        let mut flags = HashMap::new();
        if args.len() < 2 {
            return Some(flags);
        }
        let mut i = 2;
        while i<args.len() {
            let arg = args.nth(i);
            let flag = self.filter_flag_in_array(arg.unwrap());
            if flag.is_none() {
                // TODO: Log more specific errors
                return None;
            }
            let safe_flag = flag.unwrap();
            if safe_flag.has_arguments {
                let flag_arguments = self.get_flag_value(i);
                if flag_arguments.is_none() {
                    // TODO: panic with value not given error
                    return None;
                }
                flags.insert(safe_flag.clone().id, flag_arguments);
                i += 2;
            } else {
                flags.insert(safe_flag.id, None);
                i += 1;
            }

        }

        return Some(flags);
    }

    /// Gets an flag by the raw argument provided by the rust interface
    fn filter_flag_in_array(&mut self, arg: String) -> Option<Flag> {
        if arg.starts_with("--") {
            let parsed_arg = arg.split("--").collect::<Vec<&str>>()[1];
            return self.get_flag_by_id(String::from(parsed_arg));
        } else if arg.starts_with("-") {
            let parsed_arg = arg.split("-").collect::<Vec<&str>>()[1];
            return self.get_flag_by_shorthand(String::from(parsed_arg));
        }
        return None;
    }

    /// Gets the flag that is identified by the given id.
    /// If there is no flag with this ID, the result will
    /// be none
    fn get_flag_by_id(&mut self, id: String) -> Option<Flag> {
        for flag in self.flags.clone() {
            if flag.id == id {
                return Some(flag.clone());
            }
        }
        return None;
    }

    /// Gets the flag that is identified by the given shorthand.
    /// If there is no flag with this shorthand, the result will
    /// be none
    fn get_flag_by_shorthand(&mut self, shorthand: String) -> Option<Flag> {
        for flag in self.flags.clone() {
            if flag.shorthands.contains(&shorthand) {
                return Some(flag.clone());
            }
        }
        return None;
    }

    /// Gets the value of a flag if it must be provided
    fn get_flag_value(&mut self, current_index: usize) -> Option<String> {
        let mut args = std::env::args();
        let max_size = args.len();
        let next_index = current_index+1;
        if next_index > max_size {
            return None;
        }
        let value = args.nth(next_index).unwrap();
        if value.starts_with("-") {
            return None;
        }
        return Some(value);

    }

}