use yaml_rust::yaml;
use glium;

pub struct Setting {
    dimension: [u32;2],
    vsync: bool,
    multisampling: u16,
    // fullscreen: bool,
}

impl Setting {
    pub fn from_yaml(config: &yaml::Yaml) -> Result<Setting, String> {
        let hash = try!(config.as_hash().ok_or("window setting not associative array"));
        let dimension = {
            let dimension_vec = try!(try!(hash.get(&yaml::Yaml::from_str("dimension"))
                                          .ok_or("window setting must have dimension key"))
                                     .as_vec()
                                     .ok_or("window dimension must be an array"));
            if dimension_vec.len() != 2 {
                return Err("window dimension must an array of length 2".into());
            }
            (
                try!(dimension_vec[0].as_i64().ok_or("window dimension width must be an integer")),
                try!(dimension_vec[1].as_i64().ok_or("window dimension height must be an integer"))
            )
        };

        let vsync = try!(try!(hash.get(&yaml::Yaml::from_str("vsync"))
                                      .ok_or("window setting must have vsync key"))
                       .as_bool()
                       .ok_or("window setting vsync must be a bool"));

        // let fullscreen = try!(try!(hash.get(&yaml::Yaml::from_str("fullscreen"))
        //                               .ok_or("window setting must have fullscreen key"))
        //                .as_bool()
        //                .ok_or("window setting fullscreen must be a bool"));

        let multisampling = try!(try!(hash.get(&yaml::Yaml::from_str("multisampling"))
                                      .ok_or("window setting must have multisampling key"))
                       .as_i64()
                       .ok_or("window setting multisampling must be an integer")) as u16;

        Ok(Setting {
            dimension: [dimension.0 as u32, dimension.1 as u32],
            vsync: vsync,
            multisampling: multisampling,
            // fullscreen: fullscreen,
        })
    }
}

pub fn create(setting: &Setting) -> Result<glium::backend::glutin_backend::GlutinFacade, glium::GliumCreationError<glium::glutin::CreationError>> {
    use glium::DisplayBuild;

    let mut builder = glium::glutin::WindowBuilder::new()
        .with_dimensions(setting.dimension[0], setting.dimension[1])
        // .with_fullscreen(setting.fullscreen)
        .with_title(format!("ruga"));

    if setting.vsync {
        builder = builder.with_vsync();
    }
    if setting.multisampling != 0 {
        builder = builder.with_multisampling(setting.multisampling)
    }

    builder.build_glium()
}
