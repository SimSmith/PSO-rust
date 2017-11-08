extern crate rand;

use rand::Rng;


fn main() {
    println!("Hello, particle world!");

    let n_particles = 50; // 20-50
	let x_max = 5;
	let x_min = -x_max;
	let c1 = 1;
	let c2 = 0.1;
	let alpha = 1;
	let delta_t = 1;
	let r = 2; // radius
	let T = 5;
	let w = 1.5; // Inertia weight
	let beta = 0.99; // to reduce w
	let w_lower_bound = 0.4;
	let v_max = 0.15;
	let dimensions = 2;
	let swarm_best_position = (0.0, 0.0); // x^sb
	let threshold = 0.00001;

	let mut particles = initialize_particles(n_particles);
	let p = &particles[0];
	println!("One particle: {:?}", p);
	println!("and its value: {:?}", evalutate_particle(p));
}

type vector = (f32, f32);

#[derive(Debug)]
struct Particle {
	pos: vector, // x_ij
	vel: vector, // v_ij
	best_pos: vector, // x_i^bp
}

// Initialize positions and velocities of the particles pi
fn initialize_particles(n_particles: usize) -> Vec<Particle>{
	let mut particles: Vec<Particle> = Vec::with_capacity(n_particles);
    let mut rng = rand::thread_rng();
    let xmax = 5.0;

   	for _ in 0..n_particles{
	    let r1: f32 = rng.gen();
	    let r2: f32 = rng.gen();
	    let r3: f32 = rng.gen();
	    let r4: f32 = rng.gen();

		// xij = xmin +r(xmax - xmin)
	    // vij = α/deltaT(−(xmax−xmin)/2 + r(xmax−xmin))
	    // 	   or  = α(xmin + r(xmax−xmin))/deltaT, xmin=-xmax
	    let x1 = r1 * 2.0 * xmax - xmax;
	    let x2 = r2 * 2.0 * xmax - xmax;
	    let v1 = r3 * 2.0 * xmax - xmax; // alpha = deltaT = 1, xmax=-xmin
	    let v2 = r4 * 2.0 * xmax - xmax; // alpha = deltaT = 1, xmax=-xmin
	    
	    particles.push(
	    	Particle {
				pos: (x1, x2),
				vel: (v1, v2),
				best_pos: (x1, x2),
			});
   	}

    particles
}

// Evaluate each particle in the swarm, i.e.compute f(xi), i=1,...,N.
fn evalutate_particle(p: &Particle) -> f32 {
	let (x, y) = p.pos;

	(x*x + y - 11.0).powi(2) + (x + y*y - 7.0).powi(2)
}

// Update the best position of each particle, and the global best position.
fn update_best_postions() {
	unimplemented!();
}

// Update particle velocities and positions
fn update_position_and_velocity() {
	unimplemented!();
}