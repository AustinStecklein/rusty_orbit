use std::time::Instant;

pub mod tree;

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
    // Traverse the tree and accumulate force vectors
    tree.update_units(list_of_points, delta_time);
}
fn main() {
    let vec1 = tree::Vector { x: 1.0, y: 1.0 };
    let vec2 = tree::Vector { x: 1.0, y: 1.0 };
    let part1 = tree::Particle {
        position: vec1,
        velocity: vec2,
        mass: 1000.0,
    };

    let vec3: tree::Vector = tree::Vector { x: -1.0, y: -1.0 };
    let vec4 = tree::Vector { x: -1.0, y: -1.0 };
    let part2 = tree::Particle {
        position: vec3,
        velocity: vec4,
        mass: 100000.0,
    };

    // part1 and part2 are being copied here
    let mut list_of_points = vec![part1, part2];

    println!("create THE tree");
    let mut i = 0;
    let mut current_time = Instant::now();
    // un comment for tree stuff
    // loop {
    //     // currently the delta time in each loop is so small that the float is being rounded 
    //     // down to zero
    //     let delta_time = current_time - Instant::now();
    //     current_time = Instant::now();
    //     update_all_positions(&mut list_of_points, &delta_time.as_secs_f32());
    //     if i % 10000 == 0 {
    //         println!("in loop number {}", i);
    //         println!("vector {:#?}", list_of_points[0]);
    //     }
    //     i += 1;
    // }
    

}
