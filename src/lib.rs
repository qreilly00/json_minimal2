use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Json<'a> {
    Object(HashMap<&'a str,Json<'a>>),
    Array(Vec<Json<'a>>),
    String(String),
    Number(f64),
    Bool(bool),
    Null
}

// TODO: Make errors more detailed (include position and guilty char of error).
// Forward number parse error.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEnding,
    UnexpectedSymbol,
    NumberParsingError,
    Undefined
}

impl<'a> Json<'a> {
    pub fn new_object() -> Json<'a> {
        Json::Object(HashMap::new())
    }

    pub fn new_array() -> Json<'a> {
        Json::Array(Vec::new())
    }

    pub fn string_from(value: &str) -> Json<'a> {
        Json::String(String::from(value))
    }

    pub fn is_object(&self) -> bool {
        match self {
            Json::Object(_) => true,
            _ => false
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Json::Array(_) => true,
            _ => false
        }
    }

    // maybe it would be better to just panic
    pub fn insert(&mut self, name: &'a str, value: Json<'a>) {
        match self {
            Json::Object(name_value_pairs) => {

                name_value_pairs.insert(name,value);

            },
            _ => {}
        }
    }

    pub fn get(&mut self, name: &'a str) -> Option<&Json<'a>> {
        match self {
            Json::Object(name_value_pairs) => {
                name_value_pairs.get(name)
            },
            _ => {
                None
            }
        }
    }

    pub fn remove(&mut self, name: &str) {
        match self {
            Json::Object(name_value_pairs) => {

                name_value_pairs.remove(name);

            },
            _ => {}
        }
    }

    pub fn push(&mut self, new_value: Json<'a>) {
        match self {
            Json::Array(values) => {

                values.push(new_value);

            },
            _ => {}
        }
    }

    pub fn pull(&self, index: usize) -> Option<&Json<'a>> {
        match self {
            Json::Array(values) => {
                values.get(index)
            },
            _ => {
                None
            }
        }
    }

    pub fn print(&self) -> String {
        match self {
            Json::Object(name_value_pairs) => {
                let mut output = String::new();

                output.push_str("{");

                for (name,value) in name_value_pairs {
                    output.push_str(&format!("\"{}\":{},",name,value.print()));
                }

                output.pop();
                
                output.push_str("}");

                output

            },
            Json::Array(values) => {
                let mut output = String::new();

                output.push_str("[");

                for value in values {
                    output.push_str(&value.print());
                    output.push_str(",");
                }

                output.pop();

                output.push_str("]");

                output
            },
            Json::String(value) => {
                String::from(&format!("\"{}\"",value))
            },
            Json::Number(value) => {
                value.to_string()
            },
            Json::Bool(value) => {
                if *value == true {
                    return String::from("true");
                } else {
                    return String::from("false");
                }
            },
            Json::Null => {
                String::from("null")
            }
        }
    }

    pub fn parse(input: &'a String) -> Result<Json,ParseError> {
        let mut input_iter: Vec<char> = input.chars().collect();

        let mut index: usize = 0;

        while index < input_iter.len() {

            let c = &input_iter[index];

            if !c.is_ascii_whitespace() {

                if *c == '{' {

                }

                if *c == '[' {

                }

                if *c == '\"' {
                    index += 1;
                    return Self::parse_string(&mut input_iter, &mut index);
                }

                if c.is_ascii_digit() {
                    return Self::parse_number(&mut input_iter, &mut index);
                }

                if *c == 't' || *c == 'f' {
                    return Self::parse_bool(&input);
                }

                if *c == 'n' {
                    return Self::parse_null(&input);
                }

            }

            index += 1;
        }

        
            
        return Err(ParseError::UnexpectedEnding);

    }

    fn parse_object(input: &mut Vec<char>, index: &mut usize) -> Result<Json<'a>,ParseError> {
        todo!()

        // Five it a go in the same style.
    }

    fn parse_array(input: &mut Vec<char>, index: &mut usize) -> Result<Json<'a>,ParseError> {

        /*let mut array: Vec<String> = Vec::new();
        let mut tmp_string = String::new();

        while *index < input.len() {

            let c = input[*index];

            if c != ']' {
                tmp_string.push(c);
            } else {
                tmp_string.push(c);
                break;
            }

            *index += 1;
        }

        println!("{}", &tmp_string);

        /*while *index < input.len() {

            let c = input[*index];

            if !c.is_ascii_whitespace() {
                if c == ',' || c == '}' || c == ']' || c == ':' {
                    return Ok(Json::String(string));
                }
            }

            *index += 1;
        }*/

        Ok(Json::String(tmp_string))*/

        todo!()
    }

    fn parse_value(input: &mut Vec<char>, index: &mut usize)  -> Result<Json<'a>,ParseError> {
        todo!()

        // Make this the same as Self::parse.
    }

    fn parse_string(input: &mut Vec<char>, index: &mut usize) -> Result<Json<'a>,ParseError> {

        let mut string = String::new();

        while *index < input.len() {

            let c = input[*index];

            if c != '\"' {
                string.push(c);
            } else {
                break;
            }

            *index += 1;
        }

        while *index < input.len() {

            let c = input[*index];

            if !c.is_ascii_whitespace() {
                if c == ',' || c == '}' || c == ']' || c == ':' {
                    return Ok(Json::String(string));
                }
            }
    
            *index += 1;
        }

        Ok(Json::String(string))
    }

    fn parse_number(input: &mut Vec<char>, index: &mut usize) -> Result<Json<'a>,ParseError> {
        

        let mut number = String::new();

        while *index < input.len() {
            let c = input[*index];

            if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' {
                number.push(c);
            } else {
                break;
            }

            *index += 1;
        }

        if let Ok(number) = number.parse::<f64>() {

            while *index < input.len() {
                let c = input[*index];
    
                if !c.is_ascii_whitespace() {
                    if c == ',' || c == ']' || c == '}' {
                        return Ok(Json::Number(number));
                    }
                }
    
                *index += 1;
            }

            return Ok(Json::Number(number));

        } else {
            return Err(ParseError::NumberParsingError);
        }
    }

    fn parse_bool(input: &String) -> Result<Json,ParseError> {
        let input_lower = input.to_lowercase();

        for (i, character) in input_lower.chars().enumerate() {
            if character == 't' {
                if &input_lower[i..i+4] == "true" {
                    return Ok(Json::Bool(true));
                }
            }

            if character == 'f' {
                if &input_lower[i..i+5] == "false" {
                    return Ok(Json::Bool(false));
                }
            }
        }

        Err(ParseError::UnexpectedSymbol)
    }

    fn parse_null(input: &String) -> Result<Json,ParseError> {
        let input_lower = input.to_lowercase();

        for (i, character) in input_lower.chars().enumerate() {
            if character == 'n' {
                if &input_lower[i..i+4] == "null" {
                    return Ok(Json::Null);
                }
            }
        }

        Err(ParseError::UnexpectedSymbol)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut my_object = Json::new_object();

        my_object.insert("Greeting",Json::String(String::from("Hello, you!")));
        my_object.insert("Amount of days in a week",Json::Number(7.0));

        let mut days_of_the_week = Json::new_array();

        days_of_the_week.push(Json::string_from("Monday"));
        days_of_the_week.push(Json::string_from("Tuesday"));
        days_of_the_week.push(Json::string_from("Wednesday"));
        days_of_the_week.push(Json::string_from("Thursday"));
        days_of_the_week.push(Json::string_from("Friday"));
        days_of_the_week.push(Json::string_from("Saturday"));
        days_of_the_week.push(Json::string_from("Sunday"));


        my_object.insert("Days of the week",days_of_the_week);

        my_object.insert("True or false",Json::Bool(true));

        my_object.insert("Forgotten",Json::Null);

        assert_eq!("{\"Amount of days in a week\":7,\"Days of the week\":[\"Monday\",\"Tuesday\",\"Wednesday\",\"Thursday\",\"Friday\",\"Saturday\",\"Sunday\"],\"Forgotten\":null,\"True or false\":true,\"Greeting\":\"Hello, you!\"}",my_object.print());
    }

    /*#[test]
    fn parse_array() {
        let json: String = "  [ \"Hello, world!\", 42, true, null ]  ".to_string();

        let parsed = Json::parse(&json);

        assert_eq!(Ok(Json::Array(vec![Json::string_from("Hello, world!"), Json::Number(42.0), Json::Bool(true), Json::Null])),parsed);
    }*/

    #[test]
    fn parse_string() {
        let json: String = "  \"Hello, world!\"   ".to_string();

        let parsed = Json::parse(&json);

        assert_eq!(Ok(Json::string_from("Hello, world!")),parsed);
    }

    #[test]
    fn parse_number() {
        let json: String = " 1.42 ".to_string();

        let parsed = Json::parse(&json);

        assert_eq!(Ok(Json::Number(1.42)),parsed);

        let json: String = " 2e2 ".to_string();

        let parsed = Json::parse(&json);

        assert_eq!(Ok(Json::Number(200.0)),parsed);

        let json: String = " 2E2 ".to_string();

        let parsed = Json::parse(&json);

        assert_eq!(Ok(Json::Number(200.0)),parsed);
    }

    #[test]
    fn parse_bool_true() {
        let json: String = "  true  ".to_string();

        let parsed = Json::parse(&json);

        assert_eq!(Ok(Json::Bool(true)),parsed);
    }

    #[test]
    fn parse_bool_false() {
        let json: String = "  false  ".to_string();

        let parsed = Json::parse(&json);

        assert_eq!(Ok(Json::Bool(false)),parsed);
    }

    #[test]
    fn parse_null() {
        let json: String = "  null  ".to_string();

        let parsed = Json::parse(&json);

        assert_eq!(Ok(Json::Null),parsed);
    }
}
