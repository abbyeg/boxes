use boxes::find_largest_min_bounding_boxes;

fn main() {
//     let data ="\
// ----
// -**-
// -**-
// ----
// ";
    let data1 ="\
**-------***
-*--**--***-
-----***--**
-------***--";
    let boxes = find_largest_min_bounding_boxes(&data1);
    // for b in boxes.iter() {
    //     b.print_all_bounds();
    // }

}
