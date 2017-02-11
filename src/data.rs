use glob::glob;
use regex::Regex;

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

/// Takes a string query in the format "dd.mm.yy_hh:mm" and returns a HashMap with tram lines and
/// tram stations
pub fn collect_data(query: &str) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    let date_time: Vec<&str> = query.split("_").collect();
    for entry in glob("data/*.csv").unwrap() {
        let path = entry.unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_name_split: Vec<&str> = file_name.split(" ").collect();

        // Check if the date matches
        if date_time[0] != file_name_split[0] {
            continue;
        }

        // Extracts the line
        lazy_static! {
            static ref TRAM_LINE: Regex = Regex::new(r"(\d+)_alle").unwrap();
        }

        // Open the data set
        let line = TRAM_LINE.captures(file_name_split[1]).unwrap().get(1).unwrap().as_str();
        let mut file = File::open(&path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        // Read the csv
        let fc_lines: Vec<&str> = file_content.split("\n").collect();
        for fc_line in fc_lines.iter().skip(2) {

            for (index, time) in fc_line.split(";").enumerate() {
                // Just use the "ist" columns
                if index % 2 == 0 && index > 0 {
                    // If the times match:
                    if time == date_time[1] {
                        let first_sem = fc_line.find(';').unwrap();
                        let tram_station = &fc_line[..first_sem];
                        let mut stations = result.entry(line.to_owned()).or_insert(vec![]);
                        stations.push(tram_station.to_string());
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        let data = collect_data("04.02.2017_08:53");
        assert_eq!(data.len(), 16);
    }

    #[test]
    fn data_2() {
        let data = collect_data("05.02.2017_05:35");
        assert_eq!(data.len(), 13);
    }
}
