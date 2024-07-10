use std::vec;

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

    // really dump collision handler
    let mut collision_pairs = Vec::new();

    // check for collisions the long way (use tree in future?)
    //handle collisions - the dumb way
    for (i, point) in list_of_points.iter().enumerate() {
        for possible_collision in &*list_of_points {
            // we can't collide with ourselves
            if std::ptr::eq(point, possible_collision) {
                continue;
            }
            // TODO fix: just checking position and not future position
            let points_distance = point.position.get_distance(&possible_collision.position);
            // currently not early returning for found collisions
            if points_distance <= 42.0 {
                let old_mag: f32 =
                    f32::sqrt(f32::powi(point.velocity.x, 2) + f32::powi(point.velocity.y, 2))
                        * (point.mass/ (point.mass + possible_collision.mass));
                let new_velocity_vec = tree::Vector {
                    x: point.position.x - possible_collision.position.x,
                    y: point.position.y - possible_collision.position.y,
                }
                .normialize()
                .multiple(&(old_mag * 0.8));

                collision_pairs.push((i, new_velocity_vec));
            }
        }
    }
    for (index, velocity) in collision_pairs {
        list_of_points[index].velocity.x = velocity.x;
        list_of_points[index].velocity.y = velocity.y
    }

    // now update position
    for point in list_of_points {
        point.update_velocity(&0.01);
        point.update_position(&0.01);
    }
}

#[macroquad::main("Rusty orbit")]
async fn main() {
    let vec1 = tree::Vector { x: 50.0, y: -50.0 };
    let vec2 = tree::Vector { x: 0.0, y: -2.0 };
    let part1 = tree::Particle {
        position: vec1,
        velocity: vec2,
        mass: 500.0,
        g_vector: tree::Vector { x: 0.0, y: 0.0 },
    };

    let vec3 = tree::Vector {
        x: -100.0,
        y: 100.0,
    };
    let vec4 = tree::Vector { x: 0.0, y: 2.0 };
    let part2 = tree::Particle {
        position: vec3,
        velocity: vec4,
        mass: 1000.0,
        g_vector: tree::Vector { x: 0.0, y: 0.0 },
    };

    let vec5 = tree::Vector { x: 200.0, y: 0.0 };
    let vec6 = tree::Vector { x: 0.0, y: 2.0 };
    let part3 = tree::Particle {
        position: vec5,
        velocity: vec6,
        mass: 1000.0,
        g_vector: tree::Vector { x: 0.0, y: 0.0 },
    };

    // part1 and part2 are being copied here
    let mut list_of_points = vec![part1, part2, part3];
    // let mut current_time = Instant::now();

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
