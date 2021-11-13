use z3::{Solver, SatResult, Sort};
use z3::ast::*;
use z3::{Config, Context};

fn main() {
    // Create Z3 default context 
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    // Create Z3 solver
    let solver = Solver::new(&ctx);
    // Create a constant
    let z = Int::from_i64(&ctx,0);
    let i = Int::new_const(&ctx,"i");
    let arr = Array::new_const(&ctx,"arr",&Sort::int(&ctx),&Sort::int(&ctx));
    let ith : Int = Array::select(&arr,&i).as_int().unwrap();
    let zth : Int = Array::select(&arr,&z).as_int().unwrap();
    // forall i ( arr[i] > 0 )
    let fa = forall_const(&ctx,&[&i],&[],&ith.ge(&z));
    // forall i.arr[i] > 0
    solver.assert(&fa);
    // 
    solver.assert(&zth.le(&z));
    // Check it!
    let sr = solver.check();
    // Check it
    let r = match sr {
	SatResult::Unsat => "UNSAT", 
	SatResult::Sat => {
	    let model = solver.get_model().unwrap();
	    println!("MODEL: {:?}",model);
	    "SAT"
	}
	SatResult::Unknown => "UNKNOWN",	
    };
    println!("RESULT: {}",r);
}
