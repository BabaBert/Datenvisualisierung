
const DATA: &str = "../../data/geo/ncdc-merged-sfc-mntp.txt";

pub fn read_data(){
    use std::fs;

    let input = fs::read_to_string(DATA).expect("Something went wrong reading the file");

    for y in 0..72{
        for x in 0..72{

        }
    }

}