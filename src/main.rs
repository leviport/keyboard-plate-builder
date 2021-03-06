use scad::*;

fn main() {

    let rows: i32 = 50;
    let columns: i32 = 150;

    // my file which stores my output
    let mut output_scad = ScadFile::new();

    output_scad.set_detail(100);

    let mut mx_switch_hole = scad!(Union);
    mx_switch_hole.add_child(scad!(Cube(vec3(14.0, 14.0, 4.0))));
    mx_switch_hole.add_child(
        scad!(Translate(vec3(-1.0, 4.5, 0.0)); {
            scad!(Cube(vec3(16.0, 5.0, 2.6)))
        })
    );

    let mut plate_switch_holes = scad!(Union);

    for row in (0..rows).rev() {
        for i in (0..columns).rev() {
            plate_switch_holes.add_child(
                scad!(Translate(vec3(i as f32 * 19.0, row as f32 * 19.0, 0.0)); {
                    mx_switch_hole.clone()
                })
            );
        }
    }

    output_scad.add_object(plate_switch_holes.clone());

    output_scad.write_to_file(String::from("output_file.scad"));
}
