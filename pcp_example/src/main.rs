// extern crate gcollections;
// extern crate interval;
// extern crate pcp;
use gcollections::ops::*;
use interval::interval_set::*;
use interval::ops::Range;
use pcp::concept::*;
use pcp::kernel::*;
use pcp::propagators::*;
use pcp::search::search_tree_visitor::Status::*;
use pcp::search::*;
use pcp::term::*;
use pcp::variable::ops::*;

pub fn nqueens(n: usize) {
    let mut space = FDSpace::empty();

    let mut queens = vec![];
    // 2 queens can't share the same line.
    for _ in 0..n {
        queens.push(Box::new(space.vstore.alloc(IntervalSet::new(1, n as i32))) as Var<VStore>);
    }
    for i in 0..n - 1 {
        for j in i + 1..n {
            // 2 queens can't share the same diagonal.
            let q1 = (i + 1) as i32;
            let q2 = (j + 1) as i32;
            // Xi + i != Xj + j reformulated as: Xi != Xj + j - i
            space.cstore.alloc(Box::new(XNeqY::new(
                queens[i].bclone(),
                Box::new(Addition::new(queens[j].bclone(), q2 - q1)) as Var<VStore>,
            )));
            // Xi - i != Xj - j reformulated as: Xi != Xj - j + i
            space.cstore.alloc(Box::new(XNeqY::new(
                queens[i].bclone(),
                Box::new(Addition::new(queens[j].bclone(), -q2 + q1)) as Var<VStore>,
            )));
        }
    }
    // 2 queens can't share the same column.
    // join_distinct(&mut space.vstore, &mut space.cstore, queens);
    space.cstore.alloc(Box::new(Distinct::new(queens)));

    // Search step.
    let mut search = one_solution_engine();
    search.start(&space);
    let (frozen_space, status) = search.enter(space);
    let space = frozen_space.unfreeze();

    // Print result.
    match status {
        satisfiable => {
            print!(
                "{}-queens problem is satisfiable. The first solution is:\n[",
                n
            );
            for dom in space.vstore.iter() {
                // At this stage, dom.lower() == dom.upper().
                print!("{}, ", dom.lower());
            }
            println!("]");
        }
        unsatisfiable => println!("{}-queens problem is unsatisfiable.", n),
        end_of_search => println!("Search terminated or was interrupted."),
        Unknown(_) => unreachable!(
      "After the search step, the problem instance should be either satisfiable or unsatisfiable."),
    }
}

fn main() {
    nqueens(8);
}
