use std::{fs::*, io::{Read, Write}};
fn main() {
    let mut commissioner_names = String::new();
    let mut geopolitical_zones = String::new();
    let mut ministry_names = String::new();

    File::open("../files/geopolitical_zone.csv")
        .expect("Failed to read geopolitical zones.")
        .read_to_string(&mut geopolitical_zones)
        .expect("Failed to parse!");

    File::open("../files/ministry_names.csv")
        .expect("Failed to read geopolitical zones.")
        .read_to_string(&mut ministry_names)
        .expect("Failed to parse!");

    File::open("../files/comissioner_names.csv")
        .expect("Failed to read geopolitical zones.")
        .read_to_string(&mut commissioner_names)
        .expect("Failed to parse!");
    let comissioner_names_list: Vec<_> = commissioner_names.split('\n').collect();
    let geopolitical_zones_list: Vec<_> = geopolitical_zones.split('\n').collect();
    let ministry_names_list: Vec<_> = ministry_names.split('\n').collect();
    let mut merged_csv = "".to_string();
    for i in 0..comissioner_names_list.len() {
        merged_csv.push_str(format!(
            "{},{},{}\n",
            comissioner_names_list[i], ministry_names_list[i], geopolitical_zones_list[i]
        ).as_str());
    }

    File::create("../files/Ministers_Data.csv").expect("Failed to create file!").write_all(merged_csv.as_bytes()).expect("Failed to write to file!");
    println!("Successfully create minister's data!")
}
