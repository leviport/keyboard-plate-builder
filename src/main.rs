use scad::*;

fn main() {
    // my file which stores my output
    let mut my_scad = ScadFile::new();

    my_scad.set_detail(100);

    let mut mx_switch_hole = scad!(Union);

    mx_switch_hole.add_child(scad!(Cube(vec3(14.0, 14.0, 4.0))));

    mx_switch_hole.add_child(
        scad!(Translate(vec3(-1.0, 4.5, 0.0)); {
            scad!(Cube(vec3(16.0, 5.0, 2.6)))
        })
    );

    let mut plate_switch_holes = scad!(Union);

    plate_switch_holes.add_child(mx_switch_hole.clone());

    plate_switch_holes.add_child(
        scad!(Translate(vec3(19.0, 0.0, 0.0)); {
            mx_switch_hole.clone()
        })
    );

    my_scad.add_object(plate_switch_holes.clone());

    my_scad.write_to_file(String::from("output_file.scad"));
}
