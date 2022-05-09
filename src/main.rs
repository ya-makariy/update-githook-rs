use std::{env::args, process::{exit, Command}};
use regex::Regex;

const ZERO_ID: &str = "0000000000000000000000000000000000000000";
const COMMIT_MESSAGE_PATTERN: &str = r"^(build|chore|ci|docs|feat|fix|perf|refactor|style|test|Draft)?(\(([\w\$\.\*/-].*)\))??!?: (.*)|^(Revert|Merge branch)(.*)";
const ERROR_MESSAGE: &str = "GL-HOOK-ERR: Invalid commit message format!
GL-HOOK-ERR: Commit message style check failed!
GL-HOOK-ERR: You can use one of
GL-HOOK-ERR:         (build|chore|ci|docs|feat|fix|perf|refactor|Revert|style|test|Draft) types
GL-HOOK-ERR: Example:
GL-HOOK-ERR:   feat(test)!: test commit style check.
";

fn run(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd).args(args).output().expect(
        &format!("failed to execute {}", cmd)
    );

    assert!(
        output.status.success(),
        "{}", format!("error running {} {:?}", cmd, args),
    );

    String::from_utf8(output.stdout)
        .map(|x| x.trim().to_string())
        .expect("couldn't convert stdout")
}

fn check_msg(msg: &String) -> bool {
    let re = Regex::new(COMMIT_MESSAGE_PATTERN).unwrap();
    re.is_match(msg)
}

fn main() {
    let old_commit_id = args().nth(2).expect("no old commit given!");
    let commit_id = args().nth(3).expect("no new commit given!");

    if commit_id == ZERO_ID {
        exit(0);
    }

    if old_commit_id == ZERO_ID {
        let stdout = run("git", &["log", "-1", "--pretty=format:%s"]);
        if check_msg(&stdout) {
            exit(0)
        } else {
            println!("{}", ERROR_MESSAGE);
            exit(1)
        }
    } else {
        let commits = format!("{}..{}", old_commit_id, commit_id);
        let stdout =run("git", &["log", &commits, "--pretty=format:%s"]);
        if check_msg(&stdout) {
            exit(0)
        } else {
            println!("{}", ERROR_MESSAGE);
            exit(1)
        }
    }
}
