use ::std::process::Command;

const FATAL_ERROR: &str = "Failed to execute git command";

fn main() {
    check_is_git_repository();
    let empty = check_if_empty();
    if empty {
        create_initial_commit();
    }

    //check_if_clean()
    // TODO: Check if repository is clean
    // TODO: Check if it's the first commit
    // TODO: Check if the repository is clean
}

fn check_is_git_repository() {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .output()
        .expect(FATAL_ERROR);

    if !output.status.success() {
        eprintln!("Not a git repository");
        std::process::exit(1);
    }
}

fn check_if_empty() -> bool {
    let command = Command::new("git").arg("count-objects").output().expect(FATAL_ERROR);
    let output = String::from_utf8(command.stdout).expect("Failed to parse git count-objects output");
    let count = output.split_whitespace().nth(0).expect("Failed to parse git count-objects output");



    println!("count: {}", count);
    count == "0"
}

fn git_add_all() {
    let output = Command::new("git")
        .arg("add")
        .arg("--all")
        .output()
        .expect(FATAL_ERROR);

    if !output.status.success() {
        eprintln!("Failed to add all files");
        std::process::exit(1);
    }
}

fn create_initial_commit() {
    git_add_all();

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("1")
        .output()
        .expect(FATAL_ERROR);

    if !output.status.success() {
        eprintln!("Failed to create initial commit");
        std::process::exit(1);
    }
}
