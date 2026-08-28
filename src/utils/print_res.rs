use std::{any::Any, println};

pub fn get_downcast_string_result(result: Option<Box<dyn Any>>) {
    if let Some(mut res) = result {
        match res.downcast::<i32>() {
            Ok(num_val) => {
                println!("Number result: {}", *num_val);
                return;
            }
            Err(boxed) => res = boxed,
        }
        match res.downcast::<bool>() {
            Ok(bool_val) => {
                println!("Boolean result: {}", *bool_val);
                return;
            }
            Err(boxed) => res = boxed,
        }

        match res.downcast::<Vec<i32>>() {
            Ok(list_number) => {
                let num_arr_res = list_number
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                println!("Number[] result: {}", num_arr_res);
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
