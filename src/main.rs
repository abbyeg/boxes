use boxes::min_bounding_boxes;

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
    min_bounding_boxes(&data1);
}
