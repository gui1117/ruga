#[macro_use] extern crate lazy_static;
extern crate toml;

use std::collections::BTreeMap;
use std::collections::HashMap;

#[macro_export]
macro_rules! configure {
    (
        file = $file:expr;
        debug_file = $debug_file:expr;
        $($table:ident: {
            $($key:ident: $(e $string:ident[$($variante:ident),*])* $(t $value:ident)*,)*
        },)*
    )
        =>
    {
        $(#[allow(non_camel_case_types)] pub struct $table {
            $(pub $key:
              $($string)*
              $($value)*
              ,)*
        })*
        pub struct Config {
            $(pub $table: $table,)*
        }
        fn _load_config() -> Result<Config,String> {
            use std::fs::File;
            use std::io::Read;
            use std::error::Error;
            use toml;
            use configuration::FromToml;

            let file = {
                let mut file = $file;
                debug_assert!({
                    file = $debug_file;
                    true
                });
                file
            };
            let mut config_file = try!(File::open(file).map_err(|e| {
                format!("ERROR: an error occured when openning configuration file at {}{}{}",
                        file,
                        format!("\n\tdescription: {}",e.description()),
                        if let Some(cause) = e.cause() { format!("\n\tcause: {}",cause.description()) } else { String::from("") })
            }));
            let mut config_string = String::new();
            try!(config_file.read_to_string(&mut config_string).map_err(|_| "ERROR: configuration file invalid: not valid UTF-8"));
            let mut config_parser = toml::Parser::new(&*config_string);
            let mut config_table = {
                let config_table_option = config_parser.parse();
                if let Some(config_table) = config_table_option {
                    config_table
                } else {
                    let mut error = String::from("ERROR: configuration file invalid: toml parsing failed:");
                    for err in config_parser.errors {
                        error.push_str(&*format!("\n\t[{},{}] {}",err.lo,err.hi,err.desc));
                    }
                    return Err(error);
                }
            };
            let res = Config {
                $($table: {
                    let table_toml_value = try!(config_table
                                                .remove(stringify!($table))
                                                .ok_or_else(|| format!("ERROR: configuration file invalid: expect {} table",stringify!($table))));
                    if let toml::Value::Table(mut table_table) = table_toml_value {
                        let table = $table {$(
                                $key: {
                                    let value = try!(table_table
                                             .remove(stringify!($key))
                                             .ok_or_else(|| format!("ERROR: configuration file invalid: expect {}.{} key",stringify!($table),stringify!($key))));
                                    $(
                                        let variante_error = || {
                                            let mut variante = String::from("");
                                            $(
                                                variante.push_str(&*format!(" \"{}\" or",stringify!($variante)));
                                             )*
                                                variante.pop().unwrap();
                                                variante.pop().unwrap();
                                                variante.pop().unwrap();
                                                format!("ERROR: configuration file invalid: {}.{} expect{}",stringify!($table),stringify!($key),variante)
                                        };
                                        if let toml::Value::$string(string) = value {
                                            if false {
                                                String::from("")
                                            }
                                            $(
                                            else if string == String::from(stringify!($variante)) {
                                                String::from(stringify!($variante))
                                            }
                                            )*
                                            else {
                                                return Err(variante_error());
                                            }
                                        } else {
                                            return Err(variante_error());
                                        }
                                     )*
                                    $(
                                        try!($value::from_toml(&value)
                                             .map_err(|e| format!("ERROR: configuration file invalid: {}.{}{}",stringify!($table),stringify!($key),e)))
                                     )*
                                },
                                )*};

                        if !table_table.is_empty() {
                            let mut error = String::from("ERROR: configuration file invalid: unused keys:");
                            for (key,_) in table_table {
                                error.push_str(&*format!("\n\t{}.{}", stringify!($table), key));
                            }
                            return Err(error);
                        }
                        table
                    } else {
                        return Err(format!("ERROR: configuration file invalid: expect {} table",stringify!($table)));
                    }
                },)*
            };
            if !config_table.is_empty() {
                let mut error = String::from("ERROR: configuration file invalid: unused keys:");
                for (key,_) in config_table {
                    error.push_str(&*format!("\n\t{}", key));
                }
                return Err(error);
            }
            Ok(res)
        }

        lazy_static! {
            pub static ref config: Config = {
                use std::process::exit;

                match _load_config() {
                    Ok(conf) => conf,
                    Err(err) => {
                        println!("{}",err);
                        exit(1);
                    }
                }
            };
        }
    };
}

/// trait that implement from_toml
pub trait FromToml: Sized {
    /// convert toml element into a rust type,
    /// it raises an error if it is not the toml element expected
    fn from_toml(&toml::Value) -> Result<Self,String>;
}

macro_rules! toml_integer {
    ($ty:ty) => {
        impl FromToml for $ty {
            fn from_toml(val: &toml::Value) -> Result<Self,String> {
                Ok(try!(val.as_integer().ok_or(" expect integer")) as $ty)
            }
        }
    }
}
toml_integer!(u8);
toml_integer!(i8);
toml_integer!(u16);
toml_integer!(i16);
toml_integer!(u32);
toml_integer!(i32);
toml_integer!(u64);
toml_integer!(i64);
toml_integer!(usize);
toml_integer!(isize);

macro_rules! toml_float {
    ($ty:ty) => {
        impl FromToml for $ty {
            fn from_toml(val: &toml::Value) -> Result<Self,String> {
                Ok(try!(val.as_float().ok_or(" expect foat")) as $ty)
            }
        }
    }
}
toml_float!(f32);
toml_float!(f64);

impl FromToml for bool {
    fn from_toml(val: &toml::Value) -> Result<Self,String> {
        Ok(try!(val.as_bool().ok_or(" expect boolean")))
    }
}

impl FromToml for String {
    fn from_toml(val: &toml::Value) -> Result<Self,String> {
        Ok(String::from(try!(val.as_str().ok_or(" expect string"))))
    }
}

macro_rules! toml_array {
    ($n:expr => $($i:expr)+) => {
        impl<T: FromToml> FromToml for [T;$n] {
            fn from_toml(val: &toml::Value) -> Result<Self,String> {
                let array = try!(val.as_slice().ok_or(" expect array"));
                if array.len() != $n {
                    return Err(format!(" expect length of array to be {}", $n));
                }
                Ok([
                   $(
                       try!(T::from_toml(&array[$i])
                            .map_err(|e| format!("[{}]{}",$i,e))),
                   )+
                ])
            }
        }
    }
}

toml_array!(1 => 0);
toml_array!(2 => 0 1);
toml_array!(3 => 0 1 2);
toml_array!(4 => 0 1 2 3);
toml_array!(5 => 0 1 2 3 4);
toml_array!(6 => 0 1 2 3 4 5);
toml_array!(7 => 0 1 2 3 4 5 6);
toml_array!(8 => 0 1 2 3 4 5 6 7);
toml_array!(9 => 0 1 2 3 4 5 6 7 8);
toml_array!(10 => 0 1 2 3 4 5 6 7 8 9);

impl<T: FromToml> FromToml for Vec<T> {
    fn from_toml(val: &toml::Value) -> Result<Self,String> {
        let array = try!(val.as_slice().ok_or(" expect array"));
        let mut res = vec!();
        let mut i = 0;
        for elt in array {
            res.push(try!(T::from_toml(elt)
                          .map_err(|e| format!("[{}]{}",i,e))));
            i += 1;
        }
        Ok(res)
    }
}

macro_rules! toml_tuple {
    ($n:expr =>  $([$i:ident $ni:expr])+) => {
        impl<$($i: FromToml),+> FromToml for ($($i),+) {
            fn from_toml(val: &toml::Value) -> Result<Self,String> {
                let array = try!(val.as_slice().ok_or(" expect array"));
                if array.len() != $n {
                    return Err(format!(" expect length of array to be {}", $n));
                }
                Ok((
                   $(
                       try!($i::from_toml(&array[$ni])
                            .map_err(|e| format!("[{}]{}",$ni,e))),
                   )+
                ))
            }
        }
    }
}

toml_tuple!(2 => [A 0][B 1]);
toml_tuple!(3 => [A 0][B 1][C 2]);
toml_tuple!(4 => [A 0][B 1][C 2][D 3]);
toml_tuple!(5 => [A 0][B 1][C 2][D 3][E 4]);
toml_tuple!(6 => [A 0][B 1][C 2][D 3][E 4][F 5]);
toml_tuple!(7 => [A 0][B 1][C 2][D 3][E 4][F 5][G 6]);
toml_tuple!(8 => [A 0][B 1][C 2][D 3][E 4][F 5][G 6][H 7]);
toml_tuple!(9 => [A 0][B 1][C 2][D 3][E 4][F 5][G 6][H 7][I 8]);
toml_tuple!(10 => [A 0][B 1][C 2][D 3][E 4][F 5][G 6][H 7][I 8][J 9]);

// impl<T: FromToml> FromToml for Option<T> {
//     fn from_toml(val: toml::Value) -> Result<Self,String> {
//         if let toml::Value::Null = val {
//             return Ok(None);
//         }
//         match T::from_toml(val) {
//             Ok(res) => Ok(Some(res)),
//             Err(e) => Err(format!("expect null or {}",e).into())
//         }
//     }
// }

macro_rules! toml_map {
    ($t:ty: $e:expr) => {
        impl<T: FromToml> FromToml for $t {
            fn from_toml(val: &toml::Value) -> Result<Self,String> {
                let config_map = try!(val.as_table().ok_or(" expect table"));
                let mut map = $e;
                for (key,value) in config_map {

                    let value: T = try!(T::from_toml(value)
                                        .map_err(|e| format!(".{}{}",key,e)));

                    map.insert(key.clone(),value);
                }
                Ok(map)
            }
        }
    }
}

toml_map!(BTreeMap<String,T>: BTreeMap::new());
toml_map!(HashMap<String,T>: HashMap::new());

pub struct BitflagU32 {
    pub val: u32,
}
impl FromToml for BitflagU32 {
    fn from_toml(val: &toml::Value) -> Result<Self,String> {
        let err = " expect string of 1 and 0 and of length < 32";
        let mut string = String::from(try!(val.as_str().ok_or(err)));
        if string.len() > 32 { return Err(String::from(err)) }
        let mut bitval = 0;
        while let Some(chr) = string.pop() {
            match chr {
                '0' => bitval <<= 1,
                '1' => bitval = (bitval << 1) + 1,
                _ => return Err(String::from(err)),
            }
        }
        Ok(BitflagU32 {
            val: bitval,
        })
    }
}

