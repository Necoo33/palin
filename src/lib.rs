#[derive(Debug)]
pub struct AptProgram {
    pub name: String,
    pub repos: Vec<String>,
    pub version: String,
    pub core_type: String,
    pub traits: Vec<String>
}

#[derive(Debug)]
pub struct YumProgram {
    pub name: String,
    pub core_type: String,
    pub version: String,
    pub release: String,
    pub repository: String,
    pub from_repo: String,
    pub size: String,
    pub source: String,
    pub summary: String,
    pub url: String,
    pub license: String,
    pub description: String
}

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

pub fn get_apt_program(program_name: &str) -> std::result::Result<AptProgram, std::io::Error> { // Örnek bir program adı
    let apt_command = std::process::Command::new("apt")
                                        .args(&["list", "--installed"])
                                        .stdout(std::process::Stdio::piped())
                                        .spawn()
                                        .expect("apt command failed to start");
    
    let grep_command = std::process::Command::new("grep")
                                                        .arg(program_name)
                                                        .stdin(apt_command.stdout.expect("Failed to open apt stdout"))
                                                        .output();

    match grep_command {
        Ok(program) => {
            let program_output = std::str::from_utf8(&program.stdout).unwrap();

            println!("our program output: {}", program_output);

            let mut name = String::new();
            let mut repos: Vec<String> = vec![];
            let mut version = String::new();
            let mut core_type = String::new();
            let mut traits: Vec<String> = vec![];

            for line in program_output.lines() {
                if !line.starts_with(program_name) {
                    continue;
                }

                let split_the_line: Vec<&str> = line.split("now").collect();

                let name_and_repo_infos: Vec<&str> = split_the_line[0].split("/").collect();

                name = name_and_repo_infos[0].to_string();

                let repos_string: Vec<&str> = name_and_repo_infos[1].split(",").collect();

                for repo in repos_string {
                    if repo == "" {
                        continue;
                    }

                    repos.push(repo.to_string())
                }

                let other_infos: Vec<&str> = split_the_line[1].trim().split(" ").collect();

                version = other_infos[0].to_string();

                match other_infos[1] {
                    "amd64" => core_type = "64-bit".to_string(),
                    "i386" => core_type = "32-bit".to_string(),
                    "all" => core_type = "all".to_string(),
                    &_ => eprintln!("Core type couldn't spotted")
                }

                let replace_and_split_last_traits_info = other_infos[2].replace("[", "").replace("]", "");
                let replace_and_split_last_traits_info: Vec<&str> = replace_and_split_last_traits_info.split(",").collect();

                for individual_trait in replace_and_split_last_traits_info {
                    traits.push(individual_trait.to_string())
                }
            }

            return Ok(AptProgram {
                name, repos: repos.clone(), version, core_type: core_type.clone(), traits: traits.clone()
            })
        },
        Err(error) => {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, error))
        }
    }
}

pub fn list_all_apt_programs() -> std::result::Result<Vec<AptProgram>, std::io::Error>  {
    let mut all_programs: Vec<AptProgram> = vec![];
    let mut error_string = String::new();

    let get_programs_command = std::process::Command::new("apt")
                                                                                .arg("list")
                                                                                .arg("--installed")
                                                                                .output();

    match get_programs_command {
        Ok(programs) => {
            let our_command = std::str::from_utf8(&programs.stdout).unwrap();

            for line in our_command.lines() {
                if line.starts_with("Listing") {
                    continue;
                }

                let mut name = String::new();
                let mut repos: Vec<String> = vec![];
                let mut version = String::new();
                let mut core_type = String::new();
                let mut traits: Vec<String> = vec![];

                let split_the_line: Vec<&str> = line.split("now").collect();

                let name_and_repo_infos: Vec<&str> = split_the_line[0].split("/").collect();

                name = name_and_repo_infos[0].to_string();

                let repos_string: Vec<&str> = name_and_repo_infos[1].split(",").collect();

                for repo in repos_string {
                    if repo == "" {
                        continue;
                    }

                    repos.push(repo.to_string())
                }

                let other_infos: Vec<&str> = split_the_line[1].trim().split(" ").collect();

                version = other_infos[0].to_string();

                match other_infos[1] {
                    "amd64" => core_type = "64-bit".to_string(),
                    "i386" => core_type = "32-bit".to_string(),
                    "all" => core_type = "all".to_string(),
                    &_ => eprintln!("Core type couldn't spotted")
                }

                let replace_and_split_last_traits_info = other_infos[2].replace("[", "").replace("]", "");
                let replace_and_split_last_traits_info: Vec<&str> = replace_and_split_last_traits_info.split(",").collect();

                for individual_trait in replace_and_split_last_traits_info {
                    traits.push(individual_trait.to_string())
                }

                let new_apt_program = AptProgram {
                    name, repos: repos.clone(), version, core_type: core_type.clone(), traits: traits.clone()
                };

                all_programs.push(new_apt_program)
            }
        },
        Err(error) => {
            error_string = format!("{}", error);
        }
    }

    return match error_string.as_str() {
        "" => Ok(all_programs),
        &_ => Err(std::io::Error::new(std::io::ErrorKind::NotFound, error_string))
    }
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

pub fn get_yum_program(program: &str) -> std::result::Result<YumProgram, std::io::Error> {
    let get_yum_lists = std::process::Command::new("yum").arg("info").arg(program).output();

    match get_yum_lists {
        Ok(answer) => {
            let parse_answer = std::str::from_utf8(&answer.stdout).unwrap();
            let mut name = String::new();
            let mut core_type = String::new();
            let mut version = String::new();
            let mut release = String::new();
            let mut repository = String::new();
            let mut from_repo = String::new();
            let mut size = String::new();
            let mut source = String::new();
            let mut summary = String::new();
            let mut url = String::new();
            let mut license = String::new();
            let mut description = String::new();

            for (_, line) in parse_answer.lines().into_iter().enumerate() {
                let split_the_output: Vec<&str> = line.split(" :").collect::<Vec<&str>>();

                if line.starts_with("Name") {
                    name = split_the_output[1].trim().to_string();
                }

                if line.starts_with("Architecture") {
                    core_type = split_the_output[1].trim().to_string();
                }

                if line.starts_with("Version") {
                    version = split_the_output[1].trim().to_string();
                }

                if line.starts_with("Release") {
                    release = split_the_output[1].trim().to_string();
                }

                if line.starts_with("Size") {
                    size = split_the_output[1].trim().to_string();
                }

                if line.starts_with("Source") {
                    source = split_the_output[1].trim().to_string();
                }

                if line.starts_with("Repository") {
                    repository = split_the_output[1].trim().to_string();
                }

                if line.starts_with("From repo") {
                    from_repo = split_the_output[1].trim().to_string();
                }

                if line.starts_with("Summary") {
                    summary = split_the_output[1].trim().to_string();
                }

                if line.starts_with("URL") {
                    url = split_the_output[1].trim().to_string();
                }

                if line.starts_with("License") {
                    license = split_the_output[1].trim().to_string();
                }

                if line.starts_with("Description") {
                    description = split_the_output[1].trim().to_string();
                }

                if line.starts_with(" ") {
                    description.push_str(split_the_output[1].trim());
                    description.push_str(" ");
                }
            }

            Ok(YumProgram{
                name, core_type, version, release, description, summary, license, repository, from_repo, url, source, size
            })
        },
        Err(error) => {
            eprintln!("that error occured: {}", error);

            Err(std::io::Error::new(std::io::ErrorKind::NotFound, error))
        }
    }
}


pub fn list_all_yum_programs() -> std::result::Result<Vec<YumProgram>, std::io::Error> {
    let get_yum_lists = std::process::Command::new("yum").arg("info").arg("installed").output();

    match get_yum_lists {
        Ok(answer) => {
            let parse_answer = std::str::from_utf8(&answer.stdout).unwrap();
            let split_the_parsed_answer: Vec<&str> = parse_answer.split("Name").collect::<Vec<&str>>();
            
            let mut programs = vec![];

            for program in split_the_parsed_answer.into_iter() {
                let mut name = String::new();
                let mut core_type = String::new();
                let mut version = String::new();
                let mut release = String::new();
                let mut repository = String::new();
                let mut from_repo = String::new();
                let mut size = String::new();
                let mut source = String::new();
                let mut summary = String::new();
                let mut url = String::new();
                let mut license = String::new();
                let mut description = String::new();

                for (info_index, line) in program.lines().into_iter().enumerate() {
                    let split_the_output: Vec<&str> = line.split(":").collect::<Vec<&str>>();
                    
                    if info_index == 0 {
                        name = split_the_output.join("").trim().to_string();
                    }

                    if line.starts_with("Architecture") {
                        core_type = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("Version") {
                        version = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("Release") {
                        release = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("Size") {
                        size = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("Source") {
                        source = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("Repository") {
                        repository = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("From repo") {
                        from_repo = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("Summary") {
                        summary = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("URL") {
                        let split_the_output: Vec<&str> = line.split(" :").collect::<Vec<&str>>();

                        url = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("License") {
                        license = split_the_output[1].trim().to_string();
                    }

                    if line.starts_with("Description") {
                        description = split_the_output[1].trim().to_string();
                        description.push_str(" ");
                    }

                    if line.starts_with(" ") && info_index != 0 {
                        description.push_str(split_the_output[1].trim());
                        description.push_str(" ");
                    }
                }

                let yum_program = YumProgram{
                    name, core_type, version, release, description, summary, license, repository,
                    from_repo, url, source, size
                };

                programs.push(yum_program);
            }

            Ok(programs)
        },
        Err(error) => {
            eprintln!("That Error Occured: {}", error);

            Err(std::io::Error::new(std::io::ErrorKind::NotFound, error))
        }
    }
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