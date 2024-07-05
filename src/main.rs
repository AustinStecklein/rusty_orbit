use std::{ cell::RefCell, rc::Rc};
static G: f32 = 6.6743E-11; // distance in meters and mass in kg
static box_size: f32 = 2.0;
use std::mem;

#[derive(Debug)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    // return a vector with the applied scale factor
    fn multiple(&self, magnitude: &f32) -> Vector {
        Vector {
            x: self.x * magnitude,
            y: self.y * magnitude,
        }
    }

    // return a vector that is normalized to the values contained in self
    fn normialize(&self) -> Vector {
        let mag = f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2));
        Vector {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}
#[derive(Debug)]
struct Particle {
    position: Vector,
    velocity: Vector,
    mass: f32,
}

impl Particle {
    fn get_distance(&self, other_position: &Vector) -> f32 {
        // distance formula
        ((f32::powi(other_position.y - self.position.y, 2))
            + (f32::powi(other_position.x - self.position.x, 2)))
        .sqrt()
    }

    fn apply_force(&mut self, other: &Particle) {
        // first calculate the force of gravity that will other particle applies on this
        // particle
        let g_force =
            (G * self.mass * other.mass) / (f32::powi(self.get_distance(&other.position), 2));
        //create force vector
        let force_vector = Vector {
            x: other.position.x - self.position.x,
            y: other.position.y - self.position.y,
        }
        .normialize()
        .multiple(&g_force);

        //apply the force vector onto the velocity vector
        self.velocity.x = self.velocity.x + force_vector.x;
        self.velocity.y = self.velocity.y + force_vector.y;
    }

    fn update_position(&mut self, delta_time: &f32) {
        self.position.x = self.velocity.x * delta_time;
        self.position.y = self.velocity.y * delta_time;
    }
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Rc<RefCell<Tree>>>,
    center: Vector,
    particle: Option<Particle>,
    avg_mass: f32,
}

impl Tree {
    // build and initialize the tree structure
    // The side length must be given as this will remain constant.
    fn new() -> Tree {
        Tree {
            nodes: vec![],
            center: Vector{x:0.0, y:0.0},
            particle: None,
            avg_mass: 0.0,
        }
    }

    // Since building the quad tree is complex is will be the responsibility of
    // the tree struct to build and manage it. While appending if a particle
    // doesn't exist in the this tree object then add it. If it does exist
    // then split this tree in four and append the node to the tree is fits in
    fn append_node(&mut self, node: Particle) {
        match &self.particle {
            None => {
                self.particle = Some(node);
                return;
            },
            Some(particle) => {
                if particle.position.x >= self.center.x && particle.position.y >= self.center.y {
                    self.build_new_trees();
                    let old_particle = mem::replace(&mut self.particle, None).unwrap();
                    self.nodes[0].borrow_mut().append_node(old_particle);
                }
                else if particle.position.x >= self.center.x && particle.position.y < self.center.y {
                    self.build_new_trees();
                    let old_particle = mem::replace(&mut self.particle, None).unwrap();
                    self.nodes[1].borrow_mut().append_node(old_particle);
                }
                else if particle.position.x < self.center.x && particle.position.y < self.center.y {
                    self.build_new_trees();
                    let old_particle = mem::replace(&mut self.particle, None).unwrap();
                    self.nodes[2].borrow_mut().append_node(old_particle);
                }
                else {
                    self.build_new_trees();
                    let old_particle = mem::replace(&mut self.particle, None).unwrap();
                    self.nodes[3].borrow_mut().append_node(old_particle);
                }
            }
        }
        //figure out which quad to throw it in
        if node.position.x >= self.center.x && node.position.y >= self.center.y {
            self.nodes[0].borrow_mut().append_node(node);
        }
        else if node.position.x >= self.center.x && node.position.y < self.center.y {
            self.nodes[1].borrow_mut().append_node(node);
        }
        else if node.position.x < self.center.x && node.position.y < self.center.y {
            self.nodes[2].borrow_mut().append_node(node);
        }
        else {
            self.nodes[3].borrow_mut().append_node(node);
        }
    }
    
    // internal function to get an empty tree
    fn new_tree(center: Vector) -> Rc<RefCell<Tree>> {
        Rc::new(RefCell::new(Tree {
            nodes: vec![],
            center,
            particle: None,
            avg_mass: 0.0,
        }))
    }

    // A quad is made up of four parts. Assuming that the center is origin then
    // right and up is positive and left and down is negative. The new quads
    // will have the same offset just different signs. This offset can be
    // calculated from the center position of the parent square as long
    // as the first layer is pre-built
    // ______+_____
    // |2    |1    |
    //-|_____|_____|+
    // |3    | 4   |
    // |_____|_____|
    //       -
    fn build_new_trees(&mut self) {
        // doesn't matter if x or y is chosen since it is a square.
        // the absolute value is taken so the signs used to create the
        // different quads remain the same.
        let mut center_offset = f32::abs(self.center.x / 2.0);
        if center_offset == 0.0 {
            center_offset = box_size / 2.0;
        }
        // quadrant 1
        self.nodes.push(Tree::new_tree(Vector {
            x: self.center.x + center_offset,
            y: self.center.y + center_offset,
        }));
        // quadrant 2
        self.nodes.push(Tree::new_tree(Vector {
            x: self.center.x - center_offset,
            y: self.center.y + center_offset,
        }));
        // quadrant 3
        self.nodes.push(Tree::new_tree(Vector {
            x: self.center.x - center_offset,
            y: self.center.y - center_offset,
        }));
        // quadrant 4
        self.nodes.push(Tree::new_tree(Vector {
            x: self.center.x + center_offset,
            y: self.center.y - center_offset,
        }));
    }

    fn build_average_mass(&mut self)  -> f32{
        match &self.particle {
            // average mass is just the mass of the particle
            Some(particle) => {
                self.avg_mass = particle.mass;
                self.avg_mass
            },
            None => {
                if self.nodes.len() != 0 {
                    self.avg_mass = self.nodes[0].borrow_mut().build_average_mass() + 
                    self.nodes[1].borrow_mut().build_average_mass() +
                    self.nodes[2].borrow_mut().build_average_mass() +
                    self.nodes[3].borrow_mut().build_average_mass();
                }
                self.avg_mass
            },
        }
    }
}

// algorithm has a couple steps
// build tree filling in all of the objects
// traverse tree for each particle and compute the approximate gravitational force.

fn main() {
    let vec1 = Vector { x: 1.0, y: 1.0 };
    println!("test this mult {:#?}", vec1.multiple(&2.0));
    println!("test this normilize {:#?}", vec1.normialize());
    let vec2 = Vector { x: 1.0, y: 1.0 };
    let mut part1 = Particle {
        position: vec1,
        velocity: vec2,
        mass: 1.0,
    };

    let vec3: Vector = Vector { x: -1.0, y: -1.0 };
    let vec4 = Vector { x: -1.0, y: -1.0 };
    let part2 = Particle {
        position: vec3,
        velocity: vec4,
        mass: 1.0,
    };

    println!("get distance {:#?}", part1.get_distance(&part2.position));
    println!("test this pre-applied force {:#?}", part1);
    part1.apply_force(&part2);
    println!("test this post-applied force {:#?}", part1);

    println!("create THE tree");
    let mut tree = Tree::new();
    
    // build the tree
    tree.append_node(part1);
    //println!("added one item {:#?}", tree);
    tree.append_node(part2);
    println!("added second item {:#?}", tree);
    
    // calculate the average mass for each node
    tree.build_average_mass();
    println!("calculated mass {:#?}", tree);

}
