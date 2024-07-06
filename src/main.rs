

pub mod tree;

fn main() {
    let vec1 = tree::Vector { x: 1.0, y: 1.0 };
    let vec2 = tree::Vector { x: 1.0, y: 1.0 };
    let part1 = tree::Particle {
        position: vec1,
        velocity: vec2,
        mass: 1.0,
    };

    let vec3: tree::Vector = tree::Vector { x: -1.0, y: -1.0 };
    let vec4 = tree::Vector { x: -1.0, y: -1.0 };
    let part2 = tree::Particle {
        position: vec3,
        velocity: vec4,
        mass: 1.0,
    };

    // part1 and part2 are being copied here 
    let mut list_of_points = vec![part1, part2];

    println!("create THE tree");
    let mut tree = tree::Tree::new();
    // add to the tree
    tree.append_node(part1);
    tree.append_node(part2);

    // calculate the average mass for each node
    tree.build_average_mass();
    //println!("calculated mass {:#?}", tree);

    // big scary O(nlog(n)) apply forces calc time.
    // Traverse the tree and accumulate force vectors
    println!("pre update{:#?}", list_of_points);
    tree.update_units(&mut list_of_points, &1.0);
    println!("post update{:#?}", list_of_points);

}
