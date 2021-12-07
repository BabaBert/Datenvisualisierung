
const DATA: &str = "../../data/geo/ncdc-merged-sfc-mntp.txt";

pub fn read_data(){
    use std::fs;

    let _input = fs::read_to_string(DATA).expect("Something went wrong reading the file");

    for _y in 0..72{
        for _x in 0..72{

        }
    }

}