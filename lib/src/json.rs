use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::Deserialize;
use serde_json::{Error, from_str};
use std::fs::File;
use std::fs;
use super::util::Point;
use std::io::Read;
//use super::aw::{are_disjoint};

#[derive(Deserialize, Debug)]
pub struct ListOfPolygons {
        pub disjoint: bool,
        pub length: usize,
        pub polygons: Vec<Vec<Vec<Point>>>,
}

impl Serialize for ListOfPolygons {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("ListOfPolygons", 3)?;
        state.serialize_field("disjoint", &self.disjoint)?;
        state.serialize_field("length", &self.length)?;
        state.serialize_field("polygons", &self.polygons)?;
        state.end()
    }
}
/*
fn create_polygonlist<F>(func:F) -> ListOfPolygons 
    where F: Fn(usize) -> Vec<Vec<Point>> {

}
fn create_polygonlist(listsize: usize, psize: usize, polyf: fn(usize) -> Vec<Vec<Point>>) -> ListOfPolygons {
    let mut list: Vec<Vec<Vec<Point>>> = Vec::new();
    for i in 0..listsize {
        loop {
            let poly = polyf(psize);
            if are_disjoint(&poly) {
                break
            }
        }
        list.push(poly);
    }
    let list_of_poly = ListOfPolygons {

    }
    return list
}
*/

pub fn write_polygon_to_json(polys: &Vec<Vec<Point>>, disjoint: bool, ptype: String) -> std::io::Result<()> {
    let file = ptype + ".json";
    let list_polygons = if fs::metadata(&file).is_ok() {
        let mut list = read_polygon_from_json(&file).expect("failed to read from file");
        list.polygons.push(polys.to_vec());
        list
    } else  {
        ListOfPolygons {
            disjoint: disjoint,
            length : polys[0].len(),
            polygons : vec!(polys.to_vec()),
        }
    };
    println!("polygon length {}", list_polygons.length); 
    // Serialize it to a JSON string
    let j = serde_json::to_string_pretty(&list_polygons)?;
    // Print, write to a file, or send to an HTTP server.
    fs::write(file, j)?;

    Ok(())
}
pub fn read_polygon_from_json (file: &str) ->  Result<ListOfPolygons, Error> {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    //println! ("contents {}", contents);
    let polys: ListOfPolygons = from_str(&contents)?;
    Ok(polys)
}
