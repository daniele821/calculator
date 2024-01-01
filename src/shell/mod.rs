use std::process::Command;

use fraction::BigFraction;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{
    common::{self, Color},
    expression::solver::{self, CheckRules, FixRules},
};

struct Options {
    /// show result as decimal number
    show_dec: bool,
    /// decimal precision for result
    dec_len: u64,
    /// check rules to apply in calculations
    checks: Vec<CheckRules>,
    /// fix rules to apply in calculations
    fixes: Vec<FixRules>,
}

impl Options {
    const MAX_DEC_LEN: u64 = 100;

    fn default() -> Self {
        Self {
            show_dec: true,
            dec_len: 20,
            checks: vec![],
            fixes: FixRules::all().to_vec(),
        }
    }

    fn change(&mut self, line: &str) {
        let default = Self::default();
        let args = line.split_whitespace().skip(1).collect::<Vec<_>>();
        let opt = *args.first().unwrap_or(&"");
        let value = *args.get(1).unwrap_or(&"");
        let opt_err = format!("'{opt}' is not a valid option!");
        let value_err = format!("'{value}' is not a valid value!");
        match opt {
            "" => {
                *self = Options::default();
                suc(String::from("successfully resetted all options"));
            }
            "show_dec" | "show-dec" => match value {
                "" => {
                    self.show_dec = default.show_dec;
                    suc(String::from("successfully resetted 'show-dec'"));
                }
                "true" => {
                    self.show_dec = true;
                    suc(String::from("successfully setted 'show-dec' to true"));
                }
                "false" => {
                    self.show_dec = false;
                    suc(String::from("successfully setted 'show-dec' to false"));
                }
                _ => err(value_err),
            },
            "dec_len" | "dec-len" => match value {
                "" => {
                    self.dec_len = default.dec_len;
                    suc(String::from("successfully resetted 'dec-len'"));
                }
                _ => {
                    let parsed = args.get(1).unwrap_or(&"").parse::<u64>();
                    match parsed {
                        Ok(value) => {
                            if value > Self::MAX_DEC_LEN {
                                err(format!("{value} is too big!"));
                            } else {
                                self.dec_len = value;
                                suc(format!("successfully setted 'dec_len' to {value}"));
                            }
                        }
                        Err(_) => err(value_err),
                    }
                }
            },
            "checks" => match value {
                "" => {
                    self.checks = default.checks;
                    suc(String::from("successfully resetted 'checks'"));
                }
                "none" => {
                    self.checks = vec![];
                    suc(String::from("successfully setted 'checks' to none"));
                }
                "all" => {
                    self.checks = CheckRules::all();
                    suc(String::from("successfully setted 'checks' to all"));
                }
                _ => err(value_err),
            },
            "fixes" => match value {
                "" => {
                    self.fixes = default.fixes;
                    suc(String::from("successfully resetted 'fixes'"));
                }
                "none" => {
                    self.fixes = vec![];
                    suc(String::from("successfully setted 'fixes' to none"));
                }
                "all" => {
                    self.fixes = FixRules::all();
                    suc(String::from("successfully setted 'fixes' to all"));
                }
                _ => err(value_err),
            },
            _ => err(opt_err),
        }
    }

    fn as_decimal(&self, num: &BigFraction) -> String {
        let prec = self.dec_len as usize;
        format!("{:.prec$}", num)
    }
}

pub fn run() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut opt = Options::default();
    loop {
        let readline = rl.readline(&common::color(&Color::OTH, ">>> "));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                match line.split_whitespace().next().unwrap_or_default() {
                    "" => continue,
                    "exit" => break,
                    "clear" => {
                        Command::new("clear").spawn().unwrap().wait().unwrap();
                    }
                    "help" => println!("{}", help()),
                    "set" => opt.change(&line),
                    _ => match solver::resolve(&line, &opt.fixes, &opt.checks, true) {
                        Ok(res) => {
                            let title = common::color(&Color::TIT, "Solution (fraction):");
                            let res_str = common::color(&Color::SUC, &res);
                            println!("{title} {res_str}");
                            if opt.show_dec {
                                let title = common::color(&Color::TIT, "Solution (decimal):");
                                let res_str = common::color(&Color::SUC, &opt.as_decimal(&res));
                                println!("{title} {res_str}");
                            }
                            println!();
                        }
                        Err(err) => {
                            let title = common::color(&Color::TIT, "Error:");
                            let err = common::color(&Color::FAI, &err);
                            println!("{title} {err}\n");
                        }
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn help() -> String {
    String::from(
        "Commands:
  - exit                => close shell
  - clear               => clear terminal
  - help                => show this help message
  - set  [opt] [value]  => change options
  - *                   => parse as an expression

Set options:
  - show-dec [true|false]   => show/hide solution as a decimal value
  - dec-len  [(integer)]    => decimal solution precision
  - checks   [none|all]     => change CheckRules
  - fixes    [none|all]     => change FixRules
",
    )
}

fn err(msg: String) {
    println!("{}", common::color(&Color::FAI, &msg));
}

fn suc(msg: String) {
    println!("{}", common::color(&Color::SUC, &msg));
}
