use copper::prelude::*;
use totsu::prelude::*;

fn main() {
    // Example: state = [com_x, com_y, com_z]
    let mut state = vec![0.0, 0.0, 1.0]; // Initial CoM position

    // Define the QP: minimize (x - x_ref)^2 + (y - y_ref)^2
    let x_ref = 0.0;
    let y_ref = 0.0;

    // QP: min 0.5 * x^T Q x + c^T x
    let q = vec![
        vec![2.0, 0.0], // Q for x and y
        vec![0.0, 2.0],
    ];
    let c = vec![
        -2.0 * x_ref, // -2*x_ref
        -2.0 * y_ref,
    ];

    // Constraints: keep CoM within support polygon (e.g., |x| <= 0.1, |y| <= 0.05)
    let g = vec![
        vec![1.0, 0.0],   // x <= 0.1
        vec![-1.0, 0.0],  // -x <= 0.1
        vec![0.0, 1.0],   // y <= 0.05
        vec![0.0, -1.0],  // -y <= 0.05
    ];
    let h = vec![0.1, 0.1, 0.05, 0.05];

    // Set up the QP problem
    let prob = QpProb::new(
        q.clone(),
        c.clone(),
        None, // No equality constraints
        None,
        Some(g.clone()),
        Some(h.clone()),
    );

    // Real-time control loop using Copper
    let mut rt = copper::Realtime::new();
    rt.run(|_cycle| {
        // Solve QP for current state
        let mut solver = QpSolver::new();
        let res = solver.solve(&prob, None);

        if let Ok(sol) = res {
            // Apply solution to robot (here, just print)
            println!("Optimal CoM: x = {:.3}, y = {:.3}", sol[0], sol[1]);
            state[0] = sol[0];
            state[1] = sol[1];
        } else {
            println!("QP failed: {:?}", res.err());
        }

        copper::ControlFlow::Continue
    });
}