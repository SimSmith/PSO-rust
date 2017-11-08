extern crate rand;

use rand::Rng;

fn main() {
    println!("Hello, particle world!");

    let nParticles = 50; // 20-50
	let xMax = 5;
	let xMin = -xMax;
	let c1 = 1;
	let c2 = 0.1;
	let alpha = 1;
	let deltaT = 1;
	let r = 2; // radius
	let T = 5;
	let w = 1.5; // Inertia weight
	let beta = 0.99; // to reduce w
	let wLowerBound = 0.4;
	let vMax = 0.15;
	let dimensions = 2;
	let mut particles: Vec<Particle> = Vec::with_capacity(nParticles);
	let swarmBestPosition = (0.0, 0.0); // zeros(1, dimensions) + inf; // x^sb
	let threshold = 0.00001;

	let particle = Particle {
		pos: (1.0, 1.0),
		vel: (0.1, 0.2),
		best_pos: (1.0, 1.0),
	};

	println!("A particle: {:?}", particle);
	println!("A value: {:?}", evalutate_particle(&particle));
	initiate_position_and_velocity(&mut particles);
	println!("One particle: {:?}", particles[0]);
}

type vector = (f32, f32);

#[derive(Debug)]
struct Particle {
	pos: vector, // x_ij
	vel: vector, // v_ij
	best_pos: vector, // x_i^bp
}

// Initialize positions and velocities of the particles pi
fn initiate_position_and_velocity(particles: &mut Vec<Particle>){
	// xij = xmin +r(xmax - xmin)
    // vij = α/deltaT(−(xmax−xmin)/2 + r(xmax−xmin))
    // 	   or  = α(xmin + r(xmax−xmin))/deltaT, xmin=-xmax
    let mut rng = rand::thread_rng();
    let xmax = 5.0;

   	for i in (0..5){
	    let r1: f32 = rng.gen();
	    let r2: f32 = rng.gen();
	    let r3: f32 = rng.gen();
	    let r4: f32 = rng.gen();

	    let x1 = r1 * 2.0 * xmax - xmax;
	    let x2 = r2 * 2.0 * xmax - xmax;
	    let v1 = r3 * 2.0 * xmax - xmax; // alpha = deltaT = 1, xmax=-xmin
	    let v2 = r4 * 2.0 * xmax - xmax; // alpha = deltaT = 1, xmax=-xmin
	    
	    particles.push(Particle {
			pos: (x1, x2),
			vel: (v1, v2),
			best_pos: (x1, x2),
		});
   	}

    //unimplemented!();
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