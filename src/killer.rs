fn kill_pid(pid: u32) {
    use nix::{
        sys::signal::{kill, Signal},
        unistd::Pid,
    };

    let pid = Pid::from_raw(pid as i32);

    kill(pid, Signal::SIGKILL).unwrap();
}

fn timeout_killer(pid: u32, timeout: u64) {
    use std::{thread::sleep, time::Duration};

    sleep(Duration::from_millis(timeout));
    kill_pid(pid);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn start_process() -> u32 {
        use std::process::Command;

        Command::new("g++")
            .arg("-g")
            .arg("-o")
            .arg("./test_cases/bin/cpp/infinite_loop")
            .arg("./test_cases/src/cpp/infinite_loop.cpp")
            .output()
            .expect("Compile Error");

        Command::new("./test_cases/bin/cpp/infinite_loop")
            .spawn()
            .unwrap()
            .id()
    }
    #[test]
    fn test_kill_pid() {
        let pid = start_process();
        kill_pid(pid);
    }

    #[test]
    fn test_timeout_killer() {
        let pid = start_process();
        timeout_killer(pid, 1000);
    }
}
