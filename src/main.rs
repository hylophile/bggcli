use quick_xml::de::from_str;
use std::fs;
use std::io;

mod boardgame;
mod geeklist;

fn main() -> io::Result<()> {
    // // Read the XML data from the file
    // let filename = "data.xml";
    // let filename = "list.xml";

    // let xml_data = fs::read_to_string(filename)?;

    // // Parse the XML data
    // // match from_str::<boardgame::Items>(&xml_data) {
    // //     Ok(items) => {
    // //         println!("{:#?}", items);
    // //     }
    // //     Err(e) => {
    // //         eprintln!("Error parsing XML: {}", e);
    // //     }
    // // }
    // match from_str::<geeklist::Geeklist>(&xml_data) {
    //     Ok(items) => {
    //         println!("{:#?}", items);
    //     }
    //     Err(e) => {
    //         eprintln!("Error parsing XML: {}", e);
    //     }
    // }
    // Ok(())

    // let config_path = xdg_dirs
    //     .place_config_file("config.ini")
    //     .expect("cannot create configuration directory");
    // let cache_dir = xdg_dirs.get_cache_home();
    // dbg!(boardgame_cache_dir);
    // let mut config_file = File::create(config_path)?;
    // write!(&mut config_file, "configured = 1")?;
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bggcli")?;

    let boardgame_cache_dir = xdg_dirs.create_cache_directory("boardgames")?;
    Ok(())
}
