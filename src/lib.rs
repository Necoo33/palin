pub fn find_package_managers<'a>() -> Vec<&'a str> {
    let mut package_manager = vec![];

    let check_if_apt_exist = std::process::Command::new("apt").output();

    match check_if_apt_exist {
        Ok(_) => package_manager.push("apt"),
        Err(_) => ()
    }

    let check_if_yum_exist = std::process::Command::new("yum").output();

    match check_if_yum_exist {
        Ok(_) => package_manager.push("yum"),
        Err(_) => ()
    }

    let check_if_dnf_exist = std::process::Command::new("dnf").output();

    match check_if_dnf_exist {
        Ok(_) => package_manager.push("dnf"),
        Err(_) => ()
    }

    let check_if_rpm_exist = std::process::Command::new("rpm").output();

    match check_if_rpm_exist {
        Ok(_) => package_manager.push("rpm"),
        Err(_) => ()
    }

    let check_if_pacman_exist = std::process::Command::new("pacman").output();

    match check_if_pacman_exist {
        Ok(_) => package_manager.push("pacman"),
        Err(_) => ()
    }

    let check_if_emerge_exist = std::process::Command::new("emerge").arg("--info").output();

    match check_if_emerge_exist {
        Ok(_) => package_manager.push("emerge"),
        Err(_) => ()
    }

    let check_if_busybox_exist = std::process::Command::new("busybox").output();

    match check_if_busybox_exist {
        Ok(_) => package_manager.push("busybox"),
        Err(_) => ()
    }

    return package_manager
}

pub fn check_if_exist_in_apt(program_name: &str) -> bool  {
    let mut result = false;

    let get_programs_command = std::process::Command::new("apt")
                                                                                .arg("list")
                                                                                .arg("--installed")
                                                                                .output();

    match get_programs_command {
        Ok(programs) => {
            let our_command = std::str::from_utf8(&programs.stdout).unwrap();

            for line in our_command.lines() {
                let split_the_line: Vec<&str> = line.split("/").collect();

                if program_name == split_the_line[0] {
                    result = true;
                    break;
                }
            }
        },
        Err(error) => {
            eprintln!("that error occured: {}", error)
        }
    }

    return result
}

pub fn check_if_exist_in_dpkg(program_name: &str) -> bool  {
    let mut result = false;

    let get_programs_command = std::process::Command::new("dpkg")
                                                                                .arg("-l")
                                                                                .output();

    match get_programs_command {
        Ok(programs) => {
            let our_command = std::str::from_utf8(&programs.stdout).unwrap();

            for line in our_command.lines() {
                let split_the_line: Vec<&str> = line.split(" ").collect();

                let split_the_line: Vec<&str> = split_the_line.into_iter().filter(|l| *l != "").collect();

                if split_the_line.len() >= 2 {
                    if program_name == split_the_line[1] {
                        result = true;
                        break;
                    }
                }
            }
        },
        Err(error) => {
            eprintln!("that error occured: {}", error)
        }
    }

    return result
}

pub fn check_if_exist_in_dnf(program_name: &str) -> bool  {
    let mut result = false;

    let get_programs_command = std::process::Command::new("dnf")
                                                                                .arg("list")
                                                                                .arg("installed")
                                                                                .output();

    match get_programs_command {
        Ok(programs) => {
            let our_command = std::str::from_utf8(&programs.stdout).unwrap();

            for line in our_command.lines() {
                if line.starts_with(program_name) {
                    result = true;
                    break;
                }
            }
        },
        Err(error) => {
            eprintln!("that error occured: {}", error)
        }
    }

    return result
}

pub fn check_if_exist_in_yum(program_name: &str) -> bool  {
    let mut result = false;

    let get_programs_command = std::process::Command::new("yum")
                                                                                .arg("list")
                                                                                .arg("installed")
                                                                                .output();

    match get_programs_command {
        Ok(programs) => {
            let our_command = std::str::from_utf8(&programs.stdout).unwrap();

            for line in our_command.lines() {
                if line.starts_with(program_name) {
                    result = true;
                    break;
                }
            }
        },
        Err(error) => {
            eprintln!("that error occured: {}", error)
        }
    }

    return result
}

pub fn check_if_exist_in_rpm(program_name: &str) -> bool  {
    let mut result = false;

    let get_programs_command = std::process::Command::new("rpm")
                                                                                .arg("-qa")
                                                                                .output();

    match get_programs_command {
        Ok(programs) => {
            let our_command = std::str::from_utf8(&programs.stdout).unwrap();

            for line in our_command.lines() {
                if line == program_name {
                    result = true;
                    break;
                }
            }
        },
        Err(error) => {
            eprintln!("that error occured: {}", error)
        }
    }

    return result
}

pub fn check_if_exist_in_pacman(program_name: &str) -> bool  {
    let mut result = false;

    let get_programs_command = std::process::Command::new("pacman")
                                                                                .arg("-Q")
                                                                                .output();

    match get_programs_command {
        Ok(programs) => {
            let our_command = std::str::from_utf8(&programs.stdout).unwrap();

            for line in our_command.lines() {
                if line.starts_with(program_name) {
                    result = true;
                    break;
                }
            }
        },
        Err(error) => {
            eprintln!("that error occured: {}", error)
        }
    }

    return result
}

pub fn check_if_exist_in_busybox(program_name: &str) -> bool  {
    let mut result = false;

    let get_programs_command = std::process::Command::new("busybox")
                                                                                .arg("--list")
                                                                                .output();

    match get_programs_command {
        Ok(programs) => {
            let our_command = std::str::from_utf8(&programs.stdout).unwrap();

            for line in our_command.lines() {
                if program_name == line {
                    result = true;
                    break;
                }
            }
        },
        Err(error) => {
            eprintln!("that error occured: {}", error)
        }
    }

    return result
}

pub fn check_if_curl_exist() -> bool {
    let result;
    let curl_command = std::process::Command::new("curl").output();

    match curl_command {
        Ok(_) => result = true,
        Err(_) => result = false
    }

    return result
}

pub fn check_if_wget_exist() -> bool {
    let result;
    let wget_command = std::process::Command::new("wget").output();

    match wget_command {
        Ok(_) => result = true,
        Err(_) => result = false
    }

    return result
}

pub fn check_if_dig_exist() -> bool {
    let result;
    let dig_command = std::process::Command::new("dig").output();

    match dig_command {
        Ok(_) => result = true,
        Err(_) => result = false
    }

    return result
}

pub fn check_if_ip_exist() -> bool {
    let result;
    let ip_command = std::process::Command::new("ip").output();

    match ip_command {
        Ok(_) => result = true,
        Err(_) => result = false
    }

    return result
}