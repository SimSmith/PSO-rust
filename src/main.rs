extern crate rand;

use rand::Rng;


fn main() {
    println!("Hello, particle world!");

    let n_particles = 40; // 20-50
    const X_MAX: f32 = 5.0;
    const X_MIN: f32 = -5.0;
    const C1: f32 = 1.0;
    const C2: f32 = 0.1;
    const ALPHA: f32 = 1.0;
    const T_DELTA: f32 = 1.0;
    let mut w = 1.5; // Inertia weight
    const BETA: f32 = 0.99; // to reduce w
    const W_LOWER_BOUND: f32 = 0.4;
    const V_MAX: f32 = 0.15;
    //const dimensions = 2;
    let mut swarm_best_position = (X_MAX, X_MAX); // x^sb
    const THRESHOLD: f32 = 0.0001;
    const ITERATIONS: u32 = 100_000;

    // Init
    let mut particles = initialize_particles(n_particles, X_MAX, X_MIN, ALPHA, T_DELTA);
    
    for iter in 0..ITERATIONS{
        // Evaluate each particle in the swarm, i.e.compute f(x_i), i=1,...,N.
        let particle_fitnesses: Vec<f32> = particles.iter()
                                                .map(|p| evaluate_fitness(p.pos)).collect();
        let particle_best_finesses: Vec<f32> = particles.iter()
                                                .map(|p| evaluate_fitness(p.best_pos)).collect();
        
        // Update best positions
        update_best_postions(&mut particles, &mut swarm_best_position,
            &particle_fitnesses, &particle_best_finesses, iter);

        // Update positions and velocities
        w = update_position_and_velocity(
            &mut particles,
            &swarm_best_position,
            T_DELTA, C1, C2, V_MAX, w, BETA, W_LOWER_BOUND);

        if 1.0 / evaluate_fitness(swarm_best_position) < THRESHOLD {
            println!("Early exit at iteration #{:?}", iter);
            break;
        }
    }
    let f_best = evaluate_fitness(swarm_best_position);
    println!("Best position: {:?}\n, with fitness = {:?}\n, and function value = {:?}"
        , swarm_best_position, f_best, 1.0/f_best);
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
fn initialize_particles(n_particles: usize
    , x_max: f32, x_min: f32, alpha: f32, t_delta: f32
) -> Particles{
    let mut particles: Particles = Vec::with_capacity(n_particles);
    let mut rng = rand::thread_rng();

    for _ in 0..n_particles{
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();
        let r3: f32 = rng.gen();
        let r4: f32 = rng.gen();

        // xij = x_min +r(x_max - x_min)
        let x1 = x_min + r1*(x_max - x_min);
        let x2 = x_min + r2*(x_max - x_min);
        // vij = α/delta_t(-(x_max-x_min)/2 + r(x_max-x_min))
        let v1 = alpha/t_delta*(-(x_max-x_min)/2.0 + r3*(x_max-x_min));
        let v2 = alpha/t_delta*(-(x_max-x_min)/2.0 + r4*(x_max-x_min));
        
        particles.push(
            Particle {
                pos: (x1, x2),
                vel: (v1, v2),
                best_pos: (x1, x2),
            });
    }

    particles
}


fn evaluate_fitness((x, y): Vector) -> f32 {
    1.0 / ((x*x + y - 11.0).powi(2) + (x + y*y - 7.0).powi(2))
}


// Update the best position of each particle, and the global best position.
fn update_best_postions(
        particles: &mut Particles,
        swarm_best_position: &mut Vector,
        f_particles: &Vec<f32>,
        f_best_particles: &Vec<f32>,
        iteration: u32,
) {
    let mut f_swarm_best = evaluate_fitness(*swarm_best_position);

    for i in 0..particles.len(){
        if f_particles[i] > f_best_particles[i] {
            particles[i].best_pos = particles[i].pos;

            if f_particles[i] > f_swarm_best{
                *swarm_best_position = particles[i].pos;
                f_swarm_best = evaluate_fitness(*swarm_best_position);
                println!("Iter #{:?}: New best is {:?} at position {:?}"
                   , iteration, f_swarm_best, swarm_best_position);
            }
        }
    }
}


// Update particle velocities and positions
fn update_position_and_velocity(
    particles: &mut Particles,
    swarm_best_position: &Vector,
    t_delta: f32,
    c1: f32,
    c2: f32,
    v_max: f32,
    w: f32,
    beta: f32,
    w_lower_bound: f32,
) -> f32{
    let mut rng = rand::thread_rng();

    for i in 0..particles.len(){
        let q: f32 = rng.gen();
        let r: f32 = rng.gen();
        // vij = w*vij + c1*q(xijPB -xij)/deltaT +  c2*r(xjSB - xij)/deltaT
        let v_i_1 = w*particles[i].vel.0
            + c1*q*(particles[i].best_pos.0 - particles[i].pos.0)/t_delta
            + c2*r*(swarm_best_position.0-particles[i].pos.0)/t_delta;
        let v_i_2 = w*particles[i].vel.1
            + c1*q*(particles[i].best_pos.1 - particles[i].pos.1)/t_delta
            + c2*r*(swarm_best_position.1-particles[i].pos.1)/t_delta;
        // restrict |vij| < vMax
        let v_i_1 = if v_i_1.abs() < v_max {v_i_1} else {v_max};
        let v_i_2 = if v_i_2.abs() < v_max {v_i_2} else {v_max};
        particles[i].vel = (v_i_1, v_i_2);
        // xij = xij + vij*deltaT
        let x_i_1 = particles[i].pos.0 + v_i_1*t_delta;
        let x_i_2 = particles[i].pos.1 + v_i_2*t_delta;
        particles[i].pos = (x_i_1, x_i_2);
    }
    // update inetia weight
    let return_w = w * beta;
    if return_w < w_lower_bound {w_lower_bound} else {return_w}
}