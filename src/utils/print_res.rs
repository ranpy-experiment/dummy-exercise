use std::any::Any;

pub fn get_downcast_string_result(result: Option<Box<dyn Any>>) {
    if let Some(mut res) = result {
        match res.downcast::<i32>() {
            Ok(num_val) => {
                println!("Number result: {}", *num_val);
                return;
            }
            Err(boxed) => res = boxed,
        }

        match res.downcast::<&str>() {
            Ok(str_val) => {
                println!("String result: {}", str_val);
                return;
            }
            Err(boxed) => res = boxed,
        }

        match res.downcast::<String>() {
            Ok(string_val) => {
                println!("String result: {}", string_val);
                return;
            }
            Err(_) => panic!("unkown result type"),
        }
    }
}
