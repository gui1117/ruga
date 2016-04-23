macro_rules! entity {
    (
        $i:ident,$n:expr =>
        $i1:ident,$n1:expr => $ty1:ty,$tt1:tt
    ) => {
        #[derive(Debug,Clone)]
        pub struct $i {
            $i1: $ty1
        }

        impl $i {
            pub fn from_yaml(config: &yaml::Yaml) -> Result<$i,String> {
                let hash = try!(config.as_hash().ok_or(format!("{} setting must be associative array",$n)));

                let f1 = try!($tt1::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n1))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n1))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n1,e)));

                Ok($i {
                    $i1: f1,
                })
            }
        }
    };
    (
        $i:ident,$n:expr =>
        $i1:ident,$n1:expr => $ty1:ty,$tt1:tt,
        $i2:ident,$n2:expr => $ty2:ty,$tt2:tt
    ) => {
        #[derive(Debug,Clone)]
        pub struct $i {
            $i1: $ty1,
            $i2: $ty2,
        }

        impl $i {
            pub fn from_yaml(config: &yaml::Yaml) -> Result<$i,String> {
                let hash = try!(config.as_hash().ok_or(format!("{} setting must be associative array",$n)));

                let f1 = try!($tt1::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n1))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n1))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n1,e)));
                let f2 = try!($tt2::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n2))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n2))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n2,e)));

                Ok($i {
                    $i1: f1,
                    $i2: f2,
                })
            }
        }
    };
    (
        $i:ident,$n:expr =>
        $i1:ident,$n1:expr => $ty1:ty,$tt1:tt,
        $i2:ident,$n2:expr => $ty2:ty,$tt2:tt,
        $i3:ident,$n3:expr => $ty3:ty,$tt3:tt
    ) => {
        #[derive(Debug,Clone)]
        pub struct $i {
            $i1: $ty1,
            $i2: $ty2,
            $i3: $ty3,
        }

        impl $i {
            pub fn from_yaml(config: &yaml::Yaml) -> Result<$i,String> {
                let hash = try!(config.as_hash().ok_or(format!("{} setting must be associative array",$n)));

                let f1 = try!($tt1::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n1))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n1))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n1,e)));
                let f2 = try!($tt2::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n2))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n2))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n2,e)));
                let f3 = try!($tt3::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n3))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n3))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n3,e)));

                Ok($i {
                    $i1: f1,
                    $i2: f2,
                    $i3: f3,
                })
            }
        }
    };
    (
        $i:ident,$n:expr =>
        $i1:ident,$n1:expr => $ty1:ty,$tt1:tt,
        $i2:ident,$n2:expr => $ty2:ty,$tt2:tt,
        $i3:ident,$n3:expr => $ty3:ty,$tt3:tt,
        $i4:ident,$n4:expr => $ty4:ty,$tt4:tt
    ) => {
        #[derive(Debug,Clone)]
        pub struct $i {
            $i1: $ty1,
            $i2: $ty2,
            $i3: $ty3,
            $i4: $ty4,
        }

        impl $i {
            pub fn from_yaml(config: &yaml::Yaml) -> Result<$i,String> {
                let hash = try!(config.as_hash().ok_or(format!("{} setting must be associative array",$n)));

                let f1 = try!($tt1::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n1))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n1))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n1,e)));
                let f2 = try!($tt2::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n2))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n2))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n2,e)));
                let f3 = try!($tt3::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n3))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n3))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n3,e)));
                let f4 = try!($tt4::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n4))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n4))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n4,e)));

                Ok($i {
                    $i1: f1,
                    $i2: f2,
                    $i3: f3,
                    $i4: f4,
                })
            }
        }
    };
    (
        $i:ident,$n:expr =>
        $i1:ident,$n1:expr => $ty1:ty,$tt1:tt,
        $i2:ident,$n2:expr => $ty2:ty,$tt2:tt,
        $i3:ident,$n3:expr => $ty3:ty,$tt3:tt,
        $i4:ident,$n4:expr => $ty4:ty,$tt4:tt,
        $i5:ident,$n5:expr => $ty5:ty,$tt5:tt
    ) => {
        #[derive(Debug,Clone)]
        pub struct $i {
            $i1: $ty1,
            $i2: $ty2,
            $i3: $ty3,
            $i4: $ty4,
            $i5: $ty5,
        }

        impl $i {
            pub fn from_yaml(config: &yaml::Yaml) -> Result<$i,String> {
                let hash = try!(config.as_hash().ok_or(format!("{} setting must be associative array",$n)));

                let f1 = try!($tt1::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n1))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n1))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n1,e)));
                let f2 = try!($tt2::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n2))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n2))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n2,e)));
                let f3 = try!($tt3::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n3))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n3))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n3,e)));
                let f4 = try!($tt4::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n4))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n4))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n4,e)));
                let f5 = try!($tt5::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n5))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n5))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n5,e)));

                Ok($i {
                    $i1: f1,
                    $i2: f2,
                    $i3: f3,
                    $i4: f4,
                    $i5: f5,
                })
            }
        }
    };
    (
        $i:ident,$n:expr =>
        $i1:ident,$n1:expr => $ty1:ty,$tt1:tt,
        $i2:ident,$n2:expr => $ty2:ty,$tt2:tt,
        $i3:ident,$n3:expr => $ty3:ty,$tt3:tt,
        $i4:ident,$n4:expr => $ty4:ty,$tt4:tt,
        $i5:ident,$n5:expr => $ty5:ty,$tt5:tt,
        $i6:ident,$n6:expr => $ty6:ty,$tt6:tt
    ) => {
        #[derive(Debug,Clone)]
        pub struct $i {
            $i1: $ty1,
            $i2: $ty2,
            $i3: $ty3,
            $i4: $ty4,
            $i5: $ty5,
            $i6: $ty6,
        }

        impl $i {
            pub fn from_yaml(config: &yaml::Yaml) -> Result<$i,String> {
                let hash = try!(config.as_hash().ok_or(format!("{} setting must be associative array",$n)));

                let f1 = try!($tt1::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n1))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n1))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n1,e)));
                let f2 = try!($tt2::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n2))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n2))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n2,e)));
                let f3 = try!($tt3::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n3))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n3))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n3,e)));
                let f4 = try!($tt4::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n4))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n4))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n4,e)));
                let f5 = try!($tt5::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n5))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n5))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n5,e)));
                let f6 = try!($tt6::from_yaml(try!(hash.get(&yaml::Yaml::from_str($n6))
                                                   .ok_or(format!("{} setting must have {} key",$n,$n6))))
                              .map_err(|e| format!("{} {} error:{}",$n,$n6,e)));

                Ok($i {
                    $i1: f1,
                    $i2: f2,
                    $i3: f3,
                    $i4: f4,
                    $i5: f5,
                    $i6: f6,
                })
            }
        }
    };
}

