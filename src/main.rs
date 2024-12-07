use ::std::process::Command;

const FATAL_ERROR: &str = "Failed to execute git command";

struct Config {
    check_remote: bool,
}

fn main() {
    let config = Config {
        check_remote: false,
    };
    check_is_git_repository();
    if check_if_empty() {
        create_initial_commit();
    }
    if check_if_clean() {
        eprintln!("Repository is clean, nothing to do");
        std::process::exit(0);
    }

    if config.check_remote {
        check_remote();
    }

    let output = Command::new("git")
        .arg("rev-list")
        .arg("--all")
        .arg("--count")
        .output()
        .expect(FATAL_ERROR);
    let count = String::from_utf8(output.stdout)
        .expect("Failed to parse git rev-list output")
        .trim()
        .replace("\n", "")
        .parse::<i32>()
        .expect("Failed to parse git rev-list output");
    let count = count + 1;

    commit(&count.to_string());
}

fn check_remote() {
    let output = Command::new("git")
        .arg("remote")
        .output()
        .expect(FATAL_ERROR);
    if !output.status.success() {
        eprintln!("Failed to check if there is a remote repository");
        std::process::exit(1);
    }

    let remote_list = String::from_utf8(output.stdout).expect("Failed to parse git remote output");

    let remote_list = remote_list.split("\n").collect::<Vec<&str>>();

    if remote_list.is_empty() {
        eprintln!("No remote repository found");
        std::process::exit(1);
    }

    let found_origin = remote_list.iter().any(|&x| x == "origin");

    if !found_origin {
        eprintln!("No remote repository found, check the settings, or add one");
        std::process::exit(1);
    }

    let output = Command::new("git")
        .arg("fetch")
        .output()
        .expect(FATAL_ERROR);

    if !output.status.success() {
        eprintln!("Failed to fetch the remote repository");
        std::process::exit(1);
    }

    let output = Command::new("git").arg("pull").output().expect(FATAL_ERROR);

    if !output.status.success() {
        eprintln!("Failed to pull the remote repository");
        std::process::exit(1);
    }
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
    let command = Command::new("git")
        .arg("count-objects")
        .output()
        .expect(FATAL_ERROR);
    let output =
        String::from_utf8(command.stdout).expect("Failed to parse git count-objects output");
    let count = output
        .split_whitespace()
        .next()
        .expect("Failed to parse git count-objects output");

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
    commit("1")
}

fn commit(message: &str) {
    git_add_all();

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .expect(FATAL_ERROR);

    if !output.status.success() {
        eprintln!("Failed to create commit");
        std::process::exit(1);
    }
}

fn check_if_clean() -> bool {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect(FATAL_ERROR);

    if !output.status.success() {
        eprintln!("Failed to check if the repository is clean");
        std::process::exit(1);
    }

    String::from_utf8(output.stdout)
        .expect("Failed to parse git status output")
        .is_empty()
}

