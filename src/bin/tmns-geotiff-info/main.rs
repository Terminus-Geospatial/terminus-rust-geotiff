
use std::collections::LinkedList;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use log::{info, trace};
use pretty_env_logger;

use tiff::decoder::Decoder;
use tiff::ColorType;

use tmns_geotiff::geotiff::error;

#[derive(Clone)]
pub struct Config {
    pub img_path : PathBuf,
}

impl Config {

    fn parse_command_line() -> Result<Config,error::ErrorCode> {

        // Collect arguments to be parsed
        let mut args: LinkedList<String> = env::args().collect();

        // Construct return object
        let mut config = Config{
            img_path : PathBuf::new(),
        };

        // Fetch all parameters
        while args.len() > 0 {

            // Fetch the string out of the linked list
            let arg = args.pop_front();

            if let Some(a) = arg {

                let flag = String::from(a);

                // Input Path
                config.img_path = PathBuf::from( flag );

            } else {
                panic!("error fetching parameter");
            }

        }

        if config.img_path.exists() == false {
            return Err(error::ErrorCode::NotFound);
        }

        return Ok(config);
    }
}

fn main() {

    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    trace!( "Start of application");

    // Grab the command-line parameters
    let cmd_args = match Config::parse_command_line() {
        Ok(T) => T,
        Err(error::ErrorCode::NotFound) => panic!( "Unable to parse command-line options"),
    };

    // Create a file handle for image
    let img_path = &cmd_args.img_path;
    let img_file = File::open( &img_path ).expect("Unable to open file");

    let mut decoder = Decoder::new( &img_file ).expect( "Unable to create decoder" );

    ///////////////////////////////////////////////////////////
    // We need to start printing key image information here
    info!( "Image Path: {}", cmd_args.img_path.to_str().unwrap() );

    // Dimensions
    let dims = decoder.dimensions().expect("Invalid image dimensions");
    info!( "Image Size. Rows: {} pixels, Cols: {} pixels", dims.0, dims.1 );

    // Colortype
    let ctype = decoder.colortype().expect("Invalid colortype");
    info!( "Image Color Type: {:?}", ctype );

    //             assert_eq!(decoder.colortype().unwrap(), expected_type);
    //             let img_res = decoder.read_image().unwrap();
    //
    //             match img_res {
    //                 DecodingResult::$buffer(res) => {
    //                     let sum: $sum_ty = res.into_iter().map(<$sum_ty>::from).sum();
    //                     assert_eq!(sum, expected_sum);
    //                 }
    //                 _ => panic!("Wrong bit depth"),
    //             }
}