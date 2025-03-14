// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    // TODO: Something goes here
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            name: "green".to_string(),
            hex: String::from("#00FF00"),
        };

        assert_eq!(green.name, String::from("green"));
        assert_eq!(green.hex, String::from("#00FF00"));
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(String::from("green"), String::from("#00FF00"));

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct {};
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
