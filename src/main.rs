use std::io::{stdout, Write};

use rustyline::{error::ReadlineError, DefaultEditor};
use tan::{api::eval_string, context::Context, expr::Expr, util::module_util::require_module};

mod shell;

// #todo investigate if we can leverage the similarities between repl and shell.

fn main() -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;

    // #todo how to handle shell history?
    // if rl.load_history(HISTORY_FILENAME).is_err() {
    //     println!("No previous history.");
    // }

    println!("Tan, press CTRL-D to exit.");

    let mut context = Context::new();

    shell::setup_lib_shell(&mut context);
    let module = require_module("shell", &mut context);

    // #todo reuse `use` code here or extract helper!
    let bindings = module.scope.bindings.borrow().clone();
    for (name, value) in bindings {
        context.top_scope.insert(name, value.clone());
    }

    let mut index = 0;

    loop {
        // #todo what is a good shell prompt?
        // #todo should probably be customizable.
        // #todo have prefix for output/result also.
        // #todo try to use the legendary `READY` in some capacity.
        let readline = rl.readline(&format!("{index}> "));

        match readline {
            Ok(input) => {
                rl.add_history_entry(&input)?;

                // #todo find better input variable name.
                // #todo use input list/array, like wolfram, e.g. (*in* 1), nah too difficult to type!
                context
                    .scope
                    .insert(format!("$i{index}"), Expr::String(input.clone()));

                // #insight this version of eval_string does not create a new module for each input, which is what we want.
                let result = eval_string(&input, &mut context);

                let Ok(value) = result else {
                    let errors = result.unwrap_err();

                    let mut error_strings = Vec::new();
                    for error in errors {
                        // #todo this is temp.
                        error_strings
                            // .push(format!("ERROR: {}", format_error_pretty(&error, &input)));
                            .push(format!("ERROR: {}", error));
                    }

                    eprintln!("{}", error_strings.join("\n\n"));

                    continue;
                };

                // #todo find better output variable name.
                // #todo use output list/array, like wolfram, e.g. (*out* 1)
                context.scope.insert(format!("$o{index}"), value.clone());

                match value {
                    Expr::One => (),
                    _ => println!("{value}"),
                }

                let _ = stdout().flush();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("ERROR: {err:?}");
                break;
            }
        }

        index += 1;
    }

    // // #todo could we trap the (exit)?
    // rl.save_history(HISTORY_FILENAME).unwrap();

    Ok(())
}
