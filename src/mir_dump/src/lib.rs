#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(box_syntax)]

#[macro_use] extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use rustc::mir::transform::{Pass, MirPass, MirSource};
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData};
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

impl<'tcx> Visitor<'tcx> for PrintVisitor {
    fn visit_basic_block_data(&mut self, bb: BasicBlock, d: &BasicBlockData<'tcx>) {
        println!("{:?}:", bb);
        println!("{:?}", d);
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_mir_pass(box DPass);
}
