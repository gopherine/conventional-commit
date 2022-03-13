use inquire::{error::InquireResult,required, Select, Text};
use std::fmt;
use std::process::Command;

fn cli() -> InquireResult<()> {
    let ctype =get_ctype_flag(get_commit_type());
    let scope=get_input_value_req("scope", "Define scope of this commit?");
    let sdesc=get_input_value_req("short_description", "Write short imperative tense description of the change?");
    let ldesc=get_input_value_req("long_description", "Provide a longer description of the change?");
    let break_change =get_input_value("breaking_change", "list any breaking changes or issues closed by this change");

    let cmd =format!("\"{}({}): {} \n {} \n {}\"", ctype, scope, sdesc, ldesc, break_change);

    Command::new("git")
    .args(["commit","-m",&cmd])
    .output()
    .expect("failed to execute process");

    println!("git commit -m {}",cmd);
    
    Ok(())
}

fn get_ctype_flag(ctype: CommitType) -> String {
    match ctype {
        CommitType::Fix => "fix",
        CommitType::Feat => "feat",
        CommitType::Docs => "docs",
        CommitType::Style => "style",
        CommitType::Refactor => "refactor",
        CommitType::Perf => "perf",
        CommitType::Test => "test",
        CommitType::Build => "build",
        CommitType::Ci => "ci",
    }
    .into()
}

fn main() {
    cli();
}

enum CommitType {
    Fix,
    Feat,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
    Build,
    Ci,
}

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           CommitType::Fix => write!(f, "\u{1F41B} fix: Bug fix"),
           CommitType::Feat => write!(f, "\u{1F984} feat: New feature"),
           CommitType::Docs => write!(f, "\u{1F4D1} docs: Documentation changes"),
           CommitType::Style => write!(f, "\u{1F60E} style: Styling changes (no change to actual meaning of the code"),
           CommitType::Refactor => write!(f, "\u{1F6E0} refactor: Improve existing code"),
           CommitType::Perf => write!(f, "\u{1F531} perf: Improved performance"),
           CommitType::Test => write!(f, "\u{1F3E5} test: Adding test or correcting existing test"),
           CommitType::Build => write!(f, "\u{1F3CB} build: Changes that affect the build system"),
           CommitType::Ci => write!(f, "\u{1F9D8} ci: Changes to our CI configuration"),
       }
    }
}


fn get_commit_type() -> CommitType {
    let options = vec![
        CommitType::Fix,
        CommitType::Feat,
        CommitType::Docs,
        CommitType::Style,
        CommitType::Refactor,
        CommitType::Perf,
        CommitType::Test,
        CommitType::Build,
        CommitType::Ci,
    ];

    let ctype = Select::new("What type of commit is it?", options).prompt();
    match ctype {
        Ok(choice) =>  choice,
        Err(_) => panic!("commit_type:: There was an error, please try again"),
    }
}

fn get_input_value_req(input: &str,msg: &str) -> String {
    let prompt_val = Text::new(msg).with_validators(&[required!()]).prompt();
    match prompt_val {
        Ok(prompt_val) =>  {
            return prompt_val
        },
        Err(_) => panic!("An error happened when asking for {}, try again later.",input.to_string()),
    }
}

fn get_input_value(input: &str,msg: &str) -> String {
    let prompt_val = Text::new(msg).prompt();
    match prompt_val {
        Ok(prompt_val) =>  {
            if input == "breaking_change" && prompt_val.len() != 0{
                return format!("references:: {}", prompt_val)
            }
            return prompt_val
        },
        Err(_) => panic!("An error happened when asking for {}, try again later.",input.to_string()),
    }
}

/// Commit with a message
pub fn commit_msg<T: ToString>(m: T) {
	println!(
		"git commit -m \"{}\"",
		m.to_string().replace('"', "\\\"").replace('`', "\\`")
	);
}


