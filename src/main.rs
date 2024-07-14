use std::{f32::consts::PI, vec};

use macroquad::prelude::*;
// use std::time::Instant;
pub mod tree;

// current problem
// the tree and the display are in a stablish state yet collisions
// are not handled well. Currently this is "handled" by capping
// the max gravitational force that can be applied. This isn't
// my end goal with this project as I would like to see objects
// "sick" together due to gravity.

// current plan for mixing this two is that particles accumulate force
// vectors. One being the gravitational force then collision force. (hoping to reuse
// the tree for finding collision pairs). This should mean I don't need to
// cap forces as they will never get close enough to produce super large G forces

fn update_all_positions(list_of_points: &mut Vec<tree::Particle>, delta_time: &f32) {
    let mut tree = tree::Tree::new();

    // add to the tree. A copy will happen here which is required.
    // The tree needs to be constant as the list of points vector
    // is being updated
    for point in &*list_of_points {
        tree.append_node(&point);
    }
    // calculate the average mass for each node
    tree.build_average_mass();

    // big scary O(nlog(n)) apply forces calc time.
    // Traverse the tree and accumulate force vectors of gravity
    tree.calc_gravity_vector(list_of_points, delta_time);

    // now update position
    for point in list_of_points {
        point.update_velocity(&0.01);
        point.update_position(&0.01);
    }
}

fn get_number_particles(layer: &i32) -> i32 {
    if layer == &1 {
        return 1;
    }
    let large_diameter = 60 * layer + 60;
    (PI / (f32::asin(60.0 / large_diameter as f32))) as i32
}

fn map_to_position(layer: &i32, delta_theta: &f32, center: &tree::Vector) -> tree::Vector {
    if *layer == 1 {
        return *center;
    }

    let large_radius = 30 * layer + 30;
    return tree::Vector {
        x: center.x + (large_radius as f32 * f32::cos(*delta_theta)),
        y: center.y + (large_radius as f32 * f32::sin(*delta_theta)),
    };
}

fn build_mass(
    num_particles: &i32,
    center: &tree::Vector,
    velocity: &tree::Vector,
) -> Vec<tree::Particle> {
    let mut layer = 1;
    let mut num_placed = 0;
    let mut mass: Vec<tree::Particle> = vec![];

    'outer: loop {
        let current_layer_num = get_number_particles(&layer);
        for i in 0..current_layer_num {
            let delta_theta = (360 as f32 / current_layer_num as f32) * i as f32;
            let position = map_to_position(&layer, &delta_theta, center);
            // let velocity = tree::Vector{
            //     x: - center.y - position.y,
            //     y: center.x - position.x
            // }.normialize().multiple(&10.0);
            // // add particle to mass
            // let velocity = tree::Vector {
            //     x: position.x - ((center.x - position.x) * (1.0 / normal_mag)),
            //     y: position.y + ((center.y - position.y) * (1.0 / normal_mag)),
            // };

            mass.push(tree::Particle {
                position,
                velocity: *velocity,
                mass: 100.0,
                g_vector: tree::Vector { x: 0.0, y: 0.0 },
            });
            num_placed += 1;
            // if we are done then exit
            if num_placed == *num_particles {
                break 'outer;
            }
        }
        layer += 1;
    }
    return mass;
}

#[macroquad::main("Rusty orbit")]
async fn main() {
    // build two masses here that have opposite positions and velocities
    // This allows a demo of the two counter rotating
    let mut list_of_points = build_mass(
        &5,
        &tree::Vector { x: -160.0, y: 0.0 },
        &tree::Vector { x: 0.0, y: -10.0 },
    );
    let mut another_list = build_mass(
        &5,
        &tree::Vector { x: 160.0, y: 0.0 },
        &tree::Vector { x: 0.0, y: 10.0 },
    );

    list_of_points.append(&mut another_list);

    loop {
        //physics update
        // delta time isn't working right now since it is being rounded to 0
        // let delta_time = current_time - Instant::now();
        // current_time = Instant::now();
        update_all_positions(&mut list_of_points, &0.01);

        //graphics update
        clear_background(BLACK);
        for point in &list_of_points {
            draw_circle(
                screen_width() / 2.0 + point.position.x,
                screen_height() / 2.0 + point.position.y,
                30.0,
                BLUE,
            );
        }
        next_frame().await
    }
}
