// #todo find a better name.
// #todo extract the shell module.
// #todo maybe should return the output as string?
// #todo what about streaming output?
// #todo need ansi coloring
// #todo formatting/coloring should be orthogonal to rendering.

use std::{fs, sync::Arc};

use tan::{context::Context, error::Error, expr::Expr, util::module_util::require_module};

pub fn shell_help(_args: &[Expr], _context: &mut Context) -> Result<Expr, Error> {
    println!("Tan Shell");
    println!("The help file is WIP.");
    Ok(Expr::Nil)
}

// #todo somehow reuse fs/list.
pub fn shell_ls(args: &[Expr], _context: &mut Context) -> Result<Expr, Error> {
    let path = if !args.is_empty() {
        let path = args.first().unwrap();
        // let [path] = args else {
        //     return Err(Error::invalid_arguments(
        //         "`list` requires a `path` argument",
        //         None,
        //     ));
        // };

        // #todo should be Stringable
        let Some(path) = path.as_string() else {
            return Err(Error::invalid_arguments(
                "`path` argument should be a String",
                path.range(),
            ));
        };
        path
    } else {
        "."
    };

    // #todo ugh remove all unwraps!
    for entry in fs::read_dir(path).unwrap() {
        let entry_path = entry.unwrap().path();

        // #todo should this also include dirs?
        if !entry_path.is_dir() {
            println!("{}", entry_path.to_str().unwrap());
        } else {
            // #todo special formatting.
            println!("{}/", entry_path.to_str().unwrap());
        }

        // if entry_path.is_dir() {
        //     let dir_name = entry_path.to_str().unwrap().to_string();
        //     tree.push(Expr::String(format!("{dir_name}/")));
        //     tree.append(&mut walk_dir(&entry_path));
        // } else {
        //     tree.push(Expr::String(entry_path.to_str().unwrap().to_string()));
        // }
    }

    Ok(Expr::Nil)
}

// #todo shell_ls
// #todo shell_cd
// #todo shell_rm

pub fn setup_lib_shell(context: &mut Context) {
    let module = require_module("shell", context);

    module.insert("help", Expr::ForeignFunc(Arc::new(shell_help)));
    module.insert("ls", Expr::ForeignFunc(Arc::new(shell_ls)));
}

// #todo better API?
// pub fn make_shell_module(context: &mut Context) -> Module {
// }
