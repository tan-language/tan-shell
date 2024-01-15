// #todo find a better name.
// #todo extract the shell module.

use std::sync::Arc;

use tan::{context::Context, error::Error, expr::Expr, util::module_util::require_module};

pub fn shell_help(_args: &[Expr], _context: &mut Context) -> Result<Expr, Error> {
    println!("Tan Shell");
    println!("The help file is WIP.");
    Ok(Expr::One)
}

pub fn setup_lib_shell(context: &mut Context) {
    let module = require_module("shell", context);

    module.insert("help", Expr::ForeignFunc(Arc::new(shell_help)));
}
