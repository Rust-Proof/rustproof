#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(box_syntax)]

#[macro_use] extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use rustc::mir::transform::Pass;
use rustc::mir::transform::MirPass;
use rustc::mir::transform::MirSource;
use rustc::mir::repr::Mir;
use rustc::mir::repr::BasicBlock;
use rustc::mir::repr::BasicBlockData;
use rustc::mir::repr::Statement;
use rustc::mir::repr::StatementKind;
use rustc::mir::repr::Lvalue;
use rustc::mir::repr::Rvalue;
use rustc::mir::visit::Visitor;
use rustc::ty::TyCtxt;
use rustc_plugin::Registry;


struct DPass;

impl<'tcx> Pass for DPass {
}

impl<'tcx> MirPass<'tcx> for DPass {
    fn run_pass<'a>(&mut self, tcx: TyCtxt<'a, 'tcx, 'tcx>, src: MirSource, mir: &mut Mir<'tcx>) {
        // What we're doing has no effect on the dependency graph.
        let _ignore = tcx.dep_graph.in_ignore();

        // Now let's visit the code.
        PrintVisitor.visit_mir(mir)
    }
}

struct PrintVisitor;

// Calls the reports for the MIR
impl<'tcx> Visitor<'tcx> for PrintVisitor {
    fn visit_basic_block_data(&mut self, bb: BasicBlock, d: &BasicBlockData<'tcx>) {
        //println!("{:#?}:", bb);
        //println!("{:#?}", d.statements);
        //println!("{:#?}", d.statements[1].kind)
        //println!("{:#?}", d.statements.len());
		//let () = d.statements;
		printKinds(&d.statements);
    }
}

// Moves through an array of statements
pub fn printKinds(stmt: &Vec<Statement>) {
	let len = stmt.len();
	for i in 0..len {
		getVars(&stmt[i].kind);
	}
}

// Match Left and Right Variables
pub fn getVars(kind: &StatementKind) {
	match kind {
		&StatementKind::Assign(ref lValue, ref rValue)=>{
			println!("________________________________________________________________________");
			println!("Left Value = {:#?}", lValue);
			println!("Right Value = {:#?}", rValue);
			getLeftVar(&lValue);
			getRightVar(&rValue);
			println!("________________________________________________________________________");
			println!("\n\n");
		}
	}
}

// Matches the different parts of Left Value
pub fn getLeftVar(lValue: &Lvalue) {
	println!("\nLeft Variable Info");
	match lValue {
		&Lvalue::Var(ref var)=>{
			println!("Var = {:#?}", var);
		},
		&Lvalue::Temp(ref temp)=>{
			println!("Temp = {:#?}", temp);
		},
		&Lvalue::Arg(ref arg)=>{
			println!("Arg = {:#?}", arg);
		},
		&Lvalue::Static(ref stat)=>{
			println!("Static = {:#?}", stat);
		},
		_ => {}
	}
}

// Matches the different parts of Right Value
pub fn getRightVar(rValue: &Rvalue) {
	println!("\nRight Variable Info");
	match rValue {
		&Rvalue::Use(ref useA)=>{
			println!("Use = {:#?}", useA);
		},
		&Rvalue::Repeat(ref repeat, ref typed)=>{
			println!("Repeat = {:#?}", repeat);
			println!("Typed = {:#?}", typed);
		},
		&Rvalue::Cast(ref cast, ref opp, ref ty)=>{
			println!("CastKind = {:#?}", cast);
			println!("Cast Operand = {:#?}", opp);
			println!("Cast Ty = {:#?}", ty);
		},
		&Rvalue::Aggregate(ref agg, ref op)=>{
			println!("Aggregate Kind = {:#?}", agg);
			println!("Aggregate Operand = {:#?}", op);
		},
		_ => {}
	}
}


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_mir_pass(box DPass);
}
