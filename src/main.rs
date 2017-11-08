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
    let t_delta = 1;
    let w = 1.5; // Inertia weight
    let beta = 0.99; // to reduce w
    let w_lower_bound = 0.4;
    let v_max = 0.15;
    //let dimensions = 2;
    let mut swarm_best_position = (0.0, 0.0); // x^sb
    let threshold = 0.00001;

    let mut particles = initialize_particles(n_particles);
    println!("One particle: {:?}", particles[0]);
    // Evaluate each particle in the swarm, i.e.compute f(x_i), i=1,...,N.
    let particle_fitnesses: Vec<f32> = particles.iter()
                                            .map(|p| evalutate_fitness(p.pos)).collect();
    let particle_best_finesses: Vec<f32> = particles.iter()
                                            .map(|p| evalutate_fitness(p.best_pos)).collect();
    println!("and its finess: {:?}", particle_fitnesses[0]);  
}

type Vector = (f32, f32);
type Particles = Vec<Particle>;

#[derive(Debug)]
struct Particle {
    pos: Vector, // x_ij
    vel: Vector, // v_ij
    best_pos: Vector, // x_i^bp
}

// Initialize positions and velocities of the particles p_i
fn initialize_particles(n_particles: usize) -> Particles{
    let mut particles: Particles = Vec::with_capacity(n_particles);
    let mut rng = rand::thread_rng();
    const X_MAX: f32 = 5.0;
    const X_MIN: f32 = -5.0;
    const ALPHA: f32 = 1.0;
    const T_DELTA: f32 = 1.0;

    for _ in 0..n_particles{
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();
        let r3: f32 = rng.gen();
        let r4: f32 = rng.gen();

        // xij = X_MIN +r(X_MAX - X_MIN)
        let x1 = X_MIN + r1*(X_MAX - X_MIN);
        let x2 = X_MIN + r2*(X_MAX - X_MIN);
        // vij = α/delta_t(-(X_MAX-X_MIN)/2 + r(X_MAX-X_MIN))
        let v1 = ALPHA/T_DELTA*(-(X_MAX-X_MIN)/2.0 + r3*(X_MAX-X_MIN));
        let v2 = ALPHA/T_DELTA*(-(X_MAX-X_MIN)/2.0 + r4*(X_MAX-X_MIN));
        
        particles.push(
            Particle {
                pos: (x1, x2),
                vel: (v1, v2),
                best_pos: (x1, x2),
            });
    }

    particles
}

fn evalutate_fitness((x, y): Vector) -> f32 {
    (x*x + y - 11.0).powi(2) + (x + y*y - 7.0).powi(2)
}

// Update the best position of each particle, and the global best position.
fn update_best_postions(
        particles: &mut Particles,
        swarm_best_position: &mut Vector,
        f_particles: &Vec<f32>,
        f_best_particles: &Vec<f32>,
) {
    let mut f_swarm_best = evalutate_fitness(*swarm_best_position);

    for i in 0..particles.len(){
        if f_particles[i] > f_best_particles[i] {
            particles[i].best_pos = particles[i].pos;

            if f_particles[i] > f_swarm_best{
                *swarm_best_position = particles[i].pos;
                f_swarm_best = evalutate_fitness(*swarm_best_position);
            }
        }
    }
}

// Update particle velocities and positions
fn update_position_and_velocity() {
    unimplemented!();
}