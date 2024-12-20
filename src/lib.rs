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

#[derive(Debug)]
pub struct PacmanProgram {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub core_type: String,
    pub licenses: Vec<String>,
    pub groups: Vec<String>,
    pub provides: Vec<String>,
    pub depends_on: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub required_by: Vec<String>,
    pub optional_for: Vec<String>,
    pub conflicts_with: Vec<String>,
    pub replaces: Vec<String>,
    pub size: i32,
    pub packager: String,
    pub build_date: String,
    pub install_date: String,
    pub install_reason: String,
    pub install_script: String,
    pub validated_by: String
}

#[derive(Debug, Clone)]
pub struct ApkProgram {
    pub name: String,
    pub long_name: String,
    pub version: String,
    pub description: String,
    pub website: String,
    pub size: Option<i32>,
    pub update: String,
    pub sub_versions: Vec<ApkProgramSubVersion>
}

#[derive(Debug, Clone)]
pub struct ApkProgramSubVersion {
    pub name: String,
    pub description: String,
    pub website: String,
    pub size: i32
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

    let check_if_apk_exist = std::process::Command::new("apk").output();

    match check_if_apk_exist {
        Ok(_) => package_manager.push("apk"),
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

pub struct AptSourceRepo {
    pub url: String,
    pub options: Option<Vec<String>>,
    pub distro: String,
    pub parts: Vec<String>,
    pub comments: Vec<String>
}

pub fn add_source_repo_to_apt(opts: AptSourceRepo) -> std::result::Result<(), std::io::Error> {
    let create_repo_record_string = match opts.options {
        Some(options) => {
            match options.len() {
                0 => {
                    let mut parts_str = "".to_string();

                    for part in &opts.parts {
                        parts_str = format!("{} {}", parts_str, part)
                    }

                    format!("deb {} {} {}", opts.url, opts.distro, parts_str)
                },
                _ => {
                    let mut options_str = "[".to_string();

                    let length_of_options = &options.len();

                    for (index, opt) in options.into_iter().enumerate() {
                        if index + 1 == *length_of_options {
                            options_str = format!("{}]", options_str)
                        } else {
                            options_str = format!("{}, {}", options_str, opt);
                        }
                    }

                    let mut parts_str = "".to_string();

                    for part in &opts.parts {
                        parts_str = format!("{} {}", parts_str, part)
                    }

                    format!("deb {} {} {} {}", options_str, opts.url, opts.distro, parts_str)
                }
            }
        },
        None => {
            let mut parts_str = "".to_string();

            for part in &opts.parts {
                parts_str = format!("{} {}", parts_str, part)
            }

            format!("deb {} {} {}", opts.url, opts.distro, parts_str)
        }
    };

    use std::fs::OpenOptions;
    use std::io::{Read, Write};

    match OpenOptions::new().read(true).append(true).open("/etc/apt/sources.list") {
        Ok(mut sources_file) => {
            let buffer = &mut String::new();

            match sources_file.read_to_string(buffer) {
                Ok(_) => {
                    match buffer.contains(&create_repo_record_string) {
                        true => Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "That source repo already exist with exact same configurations")),
                        false => {
                            match buffer.contains(&opts.url) {
                                true => Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "That source repo already exist but with different configurations")),
                                false => {

                                    for comment in &opts.comments {
                                        let comment_string = format!("\n{}", comment);

                                        if let Err(error) = sources_file.write(comment_string.as_bytes()) {
                                            println!("error occured when we try to write comments on sources.list!");

                                            return Err(std::io::Error::new(error.kind(), error));
                                        }
                                    }

                                    match sources_file.write(format!("\n{}", create_repo_record_string).as_bytes()) {
                                        Ok(_) => Ok(()),
                                        Err(error) => Err(std::io::Error::new(error.kind(), error))
                                    }
                                }
                            }
                        }
                    }
                },
                Err(error) => {
                    println!("Error when we try to read the sources.list file!");

                    Err(std::io::Error::new(error.kind(), error))
                }
            }
        },
        Err(error) => {
            println!("error occured when we try to open sources.list file!");

            Err(std::io::Error::new(error.kind(), error))
        }
    }
}

pub fn get_yum_program(program: &str) -> std::result::Result<YumProgram, std::io::Error> {
    let check_if_yum_exist = std::process::Command::new("yum").output();
    
    let get_yum_lists;

    match check_if_yum_exist {
        Ok(_) => get_yum_lists = std::process::Command::new("yum").arg("info").arg(program).output(),
        Err(_) => get_yum_lists = std::process::Command::new("dnf").arg("info").arg(program).output()
    }

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
    let check_if_yum_exist = std::process::Command::new("yum").output();
    
    let get_yum_lists;

    match check_if_yum_exist {
        Ok(_) => get_yum_lists = std::process::Command::new("yum").arg("info").arg("installed").output(),
        Err(_) => get_yum_lists = std::process::Command::new("dnf").arg("info").arg("installed").output()
    }

    match get_yum_lists {
        Ok(answer) => {
            let parse_answer = std::str::from_utf8(&answer.stdout).unwrap();
            let split_the_parsed_answer: Vec<&str> = parse_answer.split("Name").collect::<Vec<&str>>();
            
            let mut programs = vec![];

            let mut i = 0;
            for program in split_the_parsed_answer.into_iter() {
                if i == 0 {
                    i = i + 1;
                    continue;
                }
                
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

pub fn get_pacman_program(program_name: &str) -> std::result::Result<PacmanProgram, std::io::Error> {
    let get_pacman_program = std::process::Command::new("pacman").arg("-Qi").arg(program_name).output();
    
    match get_pacman_program {
        Ok(answer) => {
            let parse_the_answer = std::str::from_utf8(&answer.stdout).unwrap();
            
            let mut name: String = "".to_string();
            let mut version: String = "".to_string();
            let mut description: String = "".to_string();
            let mut core_type: String = "".to_string();
            let mut url: String = "".to_string();
            let mut licenses: Vec<String> = vec![];
            let mut groups: Vec<String> = vec![];
            let mut provides: Vec<String> = vec![];
            let mut depends_on: Vec<String> = vec![];
            let mut optional_dependencies: Vec<String> = vec![];
            let mut required_by: Vec<String> = vec![];
            let mut optional_for: Vec<String> = vec![];
            let mut conflicts_with: Vec<String> = vec![];
            let mut replaces: Vec<String> = vec![];
            let mut size: i32 = 0;
            let mut packager: String = "".to_string();
            let mut build_date: String = "".to_string();
            let mut install_date: String = "".to_string();
            let mut install_reason: String = "".to_string();
            let mut install_script: String = "".to_string();
            let mut validated_by: String = "".to_string();
            
            for line in parse_the_answer.lines() {
                let split_the_line: Vec<&str> = line.split(" :").collect(); 
            
                if line.starts_with("Name") {
                    name = split_the_line[1].trim().to_string();       
                }

                if line.starts_with("Version") {
                    version = split_the_line[1].trim().to_string();
                }

                if line.starts_with("Description") {
                    description = split_the_line[1].trim().to_string();
                }
                 
                if line.starts_with("Architecture") {
                    let splitted_part = split_the_line[1].trim().to_string();

                    if splitted_part == "x86_64" {
                        core_type = "64 bit".to_string();
                    }

                    if splitted_part == "i686" || splitted_part == "x86" {
                        core_type = "32 bit".to_string();
                    }

                    if splitted_part == "any" {
                        core_type = "all".to_string();
                    }
                }

    
                if line.starts_with("URL") {
                    url = split_the_line[1].trim().to_string();
                }

                if line.starts_with("Licenses") {
                    let split_the_licenses_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();
                    for license in split_the_licenses_line {
                        licenses.push(license.to_string());
                    }
                }

                if line.starts_with("Groups") {
                    let split_the_groups_line = split_the_line[1].trim();

                    if split_the_groups_line == "None" {
                        continue;
                    } else {
                        let split_the_splitted_groups_line: Vec<&str> = split_the_groups_line.split("  ").collect::<Vec<&str>>();

                        for group in split_the_splitted_groups_line {
                            groups.push(group.to_string());
                        }
                    }
                }

                if line.starts_with("Provides") {
                    let split_the_provides_line = split_the_line[1].trim();

                    if split_the_provides_line == "None" {
                        continue;
                    } else {
                        let split_the_splitted_provides_line: Vec<&str> = split_the_provides_line.split("  ").collect::<Vec<&str>>();

                        for provide in split_the_splitted_provides_line {
                            provides.push(provide.to_string());
                        }
                    }
                }

                if line.starts_with("Depends On") {
                    let split_the_depends_on_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();
                        
                    if split_the_depends_on_line[0] == "None" {
                        continue;
                    }

                    for dependence in split_the_depends_on_line {
                        depends_on.push(dependence.to_string());
                    }
                }
                   
               if line.starts_with("Optional Deps") {
                    let split_the_optional_deps_line: Vec<&str> = split_the_line[1].trim().split("\n").collect::<Vec<&str>>();
                    if split_the_optional_deps_line[0].contains("None") {
                        continue;
                    }

                    optional_dependencies.push(split_the_optional_deps_line[0].to_string());

                    for (index, parsed_lines) in parse_the_answer.lines().into_iter().enumerate() {
                        if index > 9 && parsed_lines.starts_with("                  ") {
                            optional_dependencies.push(parsed_lines.trim().to_string());
                        }
                    }
               }

               if line.starts_with("Required By") {
                   let split_the_required_by_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                    if split_the_required_by_line[0] == "None" {
                        continue;
                    }

                    for requireds in split_the_required_by_line {
                        required_by.push(requireds.to_string());
                    }
               }

               if line.starts_with("Optional For") {
                    let split_the_optional_for_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                    if split_the_optional_for_line[0] == "None" {
                        continue;
                    }

                    for optionals in split_the_optional_for_line {
                        optional_for.push(optionals.to_string());
                    }
                   
               }

               
               if line.starts_with("Conflicts With") {
                   let split_the_conflicts_with_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                    if split_the_conflicts_with_line[0] == "None" {
                          continue;
                    }

                    for conflicters in split_the_conflicts_with_line {
                        conflicts_with.push(conflicters.to_string());
                    }
               }

               if line.starts_with("Replaces") {
                     let split_the_replaces_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                    if split_the_replaces_line[0] == "None" {
                        continue;
                    }

                   for replaceds in split_the_replaces_line {
                        replaces.push(replaceds.to_string());
                    }
               }

               if line.starts_with("Installed Size") {
                   let split_the_installed_size_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                   if line.contains("KiB") {
                        let split_further_the_size: Vec<&str> = split_the_installed_size_line[0].trim().split(" ").collect::<Vec<&str>>();

                        let add_one_zero_to_end = format!("{}0", split_further_the_size[0]);

                        let replace_the_string = add_one_zero_to_end.replace(".", "");

                        size = replace_the_string.parse::<i32>().unwrap();
                   }

                    
                   if line.contains("MiB") {
                         let split_further_the_size: Vec<&str> = split_the_installed_size_line[0].trim().split(" ").collect::<Vec<&str>>();

                        let add_one_zero_to_end = format!("{}0000", split_further_the_size[0]);

                        let replace_the_string = add_one_zero_to_end.replace(".", "");

                        size = replace_the_string.parse::<i32>().unwrap();
                   }

               }

                if line.starts_with("Packager") {
                   let split_the_packager_line = split_the_line[1].trim();
                    
                    packager = split_the_packager_line.to_string();
                }


                if line.starts_with("Build Date") {
                    let split_the_build_date_line = split_the_line[1].trim();
                    
                    build_date = split_the_build_date_line.to_string();
                }
                
                if line.starts_with("Install Date") {
                    let split_the_install_date_line = split_the_line[1].trim();
                    
                    install_date = split_the_install_date_line.to_string();
                }
                    
                if line.starts_with("Install Reason") {
                    let split_the_install_reason_line = split_the_line[1].trim();
                    
                    install_reason = split_the_install_reason_line.to_string();
                }

                if line.starts_with("Install Script") {
                    let split_the_install_script_line = split_the_line[1].trim();
                    
                    install_script = split_the_install_script_line.to_string();
                }

                if line.starts_with("Validated By") {
                    let split_the_validated_by_line = split_the_line[1].trim();
                    
                    validated_by = split_the_validated_by_line.to_string();
                }
            }

            let pacman_program = PacmanProgram {
                name, version, description, url, core_type, licenses, groups, provides, depends_on, optional_dependencies, optional_for, required_by, conflicts_with, replaces, size: size, packager, build_date, install_date, install_reason, install_script, validated_by
            };

            return Ok(pacman_program);
        },
        Err(error) => Err(std::io::Error::new(std::io::ErrorKind::NotFound, error))
    }        
}

pub fn list_all_pacman_programs() -> std::result::Result<Vec<PacmanProgram>, std::io::Error> {
    let get_pacman_programs = std::process::Command::new("pacman").arg("-Qi").output();
    let mut error_string = String::new();
    let mut programs = vec![];

    match get_pacman_programs {
        Ok(answer) => {
            let parse_the_answer = std::str::from_utf8(&answer.stdout).unwrap();
            let split_the_parsed_answer: Vec<&str> = parse_the_answer.split("Name            ").collect::<Vec<&str>>();

            for program in split_the_parsed_answer.clone().into_iter() {
                let mut name: String = "".to_string();
                let mut version: String = "".to_string();
                let mut description: String = "".to_string();
                let mut core_type: String = "".to_string();
                let mut url: String = "".to_string();
                let mut licenses: Vec<String> = vec![];
                let mut groups: Vec<String> = vec![];
                let mut provides: Vec<String> = vec![];
                let mut depends_on: Vec<String> = vec![];
                let mut optional_dependencies: Vec<String> = vec![];
                let mut required_by: Vec<String> = vec![];
                let mut optional_for: Vec<String> = vec![];
                let mut conflicts_with: Vec<String> = vec![];
                let mut replaces: Vec<String> = vec![];
                let mut size: i32 = 0;
                let mut packager: String = "".to_string();
                let mut build_date: String = "".to_string();
                let mut install_date: String = "".to_string();
                let mut install_reason: String = "".to_string();
                let mut install_script: String = "".to_string();
                let mut validated_by: String = "".to_string();

                for (_, line) in program.lines().into_iter().enumerate() {
                    if line.starts_with(" ") {
                        continue;
                    }

                    let split_the_line: Vec<&str> = line.split(" :").collect(); 
            
                    if line.starts_with(": ") {
                        name = split_the_line[0].trim().to_string().replace(": ", "");       
                    }

                    if line.starts_with("Version") {
                        version = split_the_line[1].trim().to_string();
                    }

                    if line.starts_with("Description") {
                        description = split_the_line[1].trim().to_string();
                    }

                    if line.starts_with("Architecture") {
                        let splitted_part = split_the_line[1].trim().to_string();

                        if splitted_part == "x86_64" {
                            core_type = "64 bit".to_string();
                        }

                        if splitted_part == "i686" || splitted_part == "x86" {
                            core_type = "32 bit".to_string();
                        }

                        if splitted_part == "any" {
                            core_type = "all".to_string();
                        }
                    }

                    if line.starts_with("URL") {
                        url = split_the_line[1].trim().to_string();
                    }

                    if line.starts_with("Licenses") {
                        let split_the_licenses_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                        for license in split_the_licenses_line {
                            licenses.push(license.to_string());
                        }
                    }

                    if line.starts_with("Groups") {
                        let split_the_groups_line = split_the_line[1].trim();

                        if split_the_groups_line == "None" {
                            continue;
                        } else {
                            let split_the_splitted_groups_line: Vec<&str> = split_the_groups_line.split("  ").collect::<Vec<&str>>();

                            for group in split_the_splitted_groups_line {
                                groups.push(group.to_string());
                            }
                        }
                    }

                    if line.starts_with("Provides") {
                        let split_the_provides_line = split_the_line[1].trim();

                        if split_the_provides_line == "None" {
                            continue;
                        } else {
                            let split_the_splitted_provides_line: Vec<&str> = split_the_provides_line.split("  ").collect::<Vec<&str>>();

                            for provide in split_the_splitted_provides_line {
                                provides.push(provide.to_string());
                            }
                        }
                    }

                    if line.starts_with("Depends On") {
                        let split_the_depends_on_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();
                        
                        if split_the_depends_on_line[0] == "None" {
                            continue;
                        }

                        for dependence in split_the_depends_on_line {
                            depends_on.push(dependence.to_string());
                        }
                    }
                   
                   if line.starts_with("Optional Deps") {
                        let split_the_optional_deps_line: Vec<&str> = split_the_line[1].trim().split("\n").collect::<Vec<&str>>();

                        let stopl1 = split_the_optional_deps_line[0].trim();

                        if stopl1 == "None" {
                            continue;
                        }

                        let format_the_starting_name = format!(": {}\n", name);

                        for parsed_entity in &split_the_parsed_answer {
                            if parsed_entity.starts_with(&format_the_starting_name) {

                                let split_the_parsed_entity = parsed_entity.split("Required By     : ").collect::<Vec<&str>>()[0];

                                optional_dependencies.push(stopl1.to_string());

                                for opt_dep_lines in split_the_parsed_entity.lines() {
                                    if !opt_dep_lines.starts_with("                  ") {
                                        continue;
                                    }

                                    optional_dependencies.push(opt_dep_lines.trim().to_string());
                                }   
                            }
                        }
                   }

                   if line.starts_with("Required By") {
                        let split_the_required_by_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                        if split_the_required_by_line[0] == "None" {
                            continue;
                        }

                        for requireds in split_the_required_by_line {
                            required_by.push(requireds.to_string());
                        }
                   }

                   if line.starts_with("Optional For") {
                        let split_the_optional_for_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                        if split_the_optional_for_line[0] == "None" {
                            continue;
                        }

                        for optionals in split_the_optional_for_line {
                            optional_for.push(optionals.to_string());
                        }
                   }

                   if line.starts_with("Conflicts With") {
                        let split_the_conflicts_with_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                        if split_the_conflicts_with_line[0] == "None" {
                            continue;
                        }

                        for conflicters in split_the_conflicts_with_line {
                            conflicts_with.push(conflicters.to_string());
                        }
                   }

                   if line.starts_with("Replaces") {
                        let split_the_replaces_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                        if split_the_replaces_line[0] == "None" {
                            continue;
                        }

                        for replaceds in split_the_replaces_line {
                            replaces.push(replaceds.to_string());
                        }
                   }

                   if line.starts_with("Installed Size") {
                        let split_the_installed_size_line: Vec<&str> = split_the_line[1].trim().split("  ").collect::<Vec<&str>>();

                        if line.contains("KiB") {
                            let split_further_the_size: Vec<&str> = split_the_installed_size_line[0].trim().split(" ").collect::<Vec<&str>>();

                            let add_one_zero_to_end = format!("{}0", split_further_the_size[0]);

                            let replace_the_string = add_one_zero_to_end.replace(".", "");

                            size = replace_the_string.parse::<i32>().unwrap();
                        }

                        if line.contains("MiB") {
                            let split_further_the_size: Vec<&str> = split_the_installed_size_line[0].trim().split(" ").collect::<Vec<&str>>();

                            let add_one_zero_to_end = format!("{}0000", split_further_the_size[0]);

                            let replace_the_string = add_one_zero_to_end.replace(".", "");

                            size = replace_the_string.parse::<i32>().unwrap();
                        }
                   }

                   if line.starts_with("Packager") {
                        let split_the_packager_line = split_the_line[1].trim();
                    
                        packager = split_the_packager_line.to_string();
                   }


                   if line.starts_with("Build Date") {
                        let split_the_build_date_line = split_the_line[1].trim();
                    
                        build_date = split_the_build_date_line.to_string();
                   }
                
                   if line.starts_with("Install Date") {
                        let split_the_install_date_line = split_the_line[1].trim();
                    
                        install_date = split_the_install_date_line.to_string();
                   }
                    
                   if line.starts_with("Install Reason") {
                        let split_the_install_reason_line = split_the_line[1].trim();
                    
                        install_reason = split_the_install_reason_line.to_string();
                   }

                   if line.starts_with("Install Script") {
                       let split_the_install_script_line = split_the_line[1].trim();
                    
                        install_script = split_the_install_script_line.to_string();
                   }

                   if line.starts_with("Validated By") {
                        let split_the_validated_by_line = split_the_line[1].trim();
                    
                        validated_by = split_the_validated_by_line.to_string();
                   }
                }

                let pacman_program = PacmanProgram {
                    name, version, description, url, core_type, licenses, groups, provides, depends_on, optional_dependencies, optional_for, required_by, conflicts_with, replaces, size: size, packager, build_date, install_date, install_reason, install_script, validated_by
                };

                programs.push(pacman_program);
            }
        },
        Err(error) => {
            eprintln!("this error occured: {}", error);
            
            error_string = error.to_string();
        }
    }

    return match error_string.as_str() {
        "" => Ok(programs),
        &_ => Err(std::io::Error::new(std::io::ErrorKind::NotFound, error_string))
    };
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

pub fn check_if_exist_in_apk(program_name: &str) -> bool  {
    let mut result = false;

    let get_programs_command = std::process::Command::new("apk")
                                                                                .arg("info")
                                                                                .arg("-vv")
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

pub fn get_apk_program(program_name: &str) -> std::result::Result<ApkProgram, std::io::Error> {
    let get_program_command = std::process::Command::new("apk")
                                                        .arg("info")
                                                        .arg("-vv")
                                                        .arg(program_name)
                                                        .output();

    match get_program_command {
        Ok(output) => {
            let our_output = std::str::from_utf8(&output.stdout).unwrap();
            let mut long_name = "".to_string();
            let mut version = "".to_string();
            let mut description = "".to_string();
            let mut website = "".to_string();
            let mut size = "".to_string();
            let mut update_info = "".to_string();

            let mut get_individual_programs = our_output
                                                        .lines()
                                                        .into_iter()
                                                        .map(|line| line.split(": ").collect::<Vec<&str>>()[0])
                                                        .collect::<Vec<&str>>();

            let mut prev_item: &str = "";
            get_individual_programs.retain(|current_item| {
                if prev_item == "" {
                    prev_item = *current_item;
                    return true
                } else {
                    if prev_item != *current_item {
                        prev_item = *current_item;
                        return true
                    } else {
                        return false;
                    }
                }
            });

            let mut subversions: Vec<ApkProgramSubVersion> = vec![];

            for pr in get_individual_programs.into_iter() {
                let mut sub_program_name = "".to_string();
                let mut sub_program_description = "".to_string();
                let mut sub_program_website = "".to_string();
                let mut sub_program_size = "".to_string();

                for line in our_output.lines().into_iter() {
                    let split_the_line = line.split(": ").collect::<Vec<&str>>();
                    let first_part_of_line = split_the_line[0].trim();
                    let second_part_of_line = split_the_line[1].trim();

                    if first_part_of_line == program_name {
                        if second_part_of_line.starts_with("http") {
                            website = second_part_of_line.to_string();
                        } else if second_part_of_line.contains("MiB") {
                            size = format!("{}000000", second_part_of_line.split(" ").collect::<Vec<&str>>()[0]);
                        } else if second_part_of_line.contains("KiB") {
                            size = format!("{}000", second_part_of_line.split(" ").collect::<Vec<&str>>()[0]);
                        }
                    } else {
                        sub_program_name = pr.to_string();

                        if second_part_of_line.starts_with("http") {
                            sub_program_website = second_part_of_line.to_string();
                        }   else if second_part_of_line.contains("MiB") {
                            sub_program_size = format!("{}000000", second_part_of_line.split(" ").collect::<Vec<&str>>()[0]);
                        } else if second_part_of_line.contains("KiB") {
                            sub_program_size = format!("{}000", second_part_of_line.split(" ").collect::<Vec<&str>>()[0]);
                        } else {
                            sub_program_description = second_part_of_line.to_string();
                        }
                    }
                }

                if sub_program_name != "".to_string() &&
                   sub_program_description != "".to_string() &&
                   sub_program_website != "".to_string() &&
                   sub_program_size != "".to_string() {
                    subversions.push(ApkProgramSubVersion {
                        name: sub_program_name,
                        description: sub_program_description,
                        website: sub_program_website,
                        size: sub_program_size.parse::<i32>().unwrap()
                    })
                }
            }

            let get_program_command = std::process::Command::new("apk")
                                                                .arg("info")
                                                                .arg("-vv")
                                                                .output()
                                                                .unwrap();

            let get_program_command = std::str::from_utf8(&get_program_command.stdout).unwrap();

            for line in get_program_command.lines() {
                if line.starts_with(program_name) {
                    let splitted_line = line.split(" - ").collect::<Vec<&str>>();

                    description = splitted_line[1].to_string();

                    let split_the_splitted_line = splitted_line[0].split("-").collect::<Vec<&str>>();
                    let length_of_the_split = split_the_splitted_line.len();

                    for (index, split) in split_the_splitted_line.clone().into_iter().enumerate() {
                        if index + 1 == length_of_the_split {
                            update_info = split.to_string()
                        }

                        if split.chars().next().map_or(false, |char| char.is_numeric()) {
                            version = split.to_string();
                        }
                    };

                    update_info = split_the_splitted_line[length_of_the_split-1].to_string();

                    long_name = split_the_splitted_line.join("-");

                    break;
                } else {
                    continue;
                }
            }

            subversions.retain(|item| {
                if item.name == program_name {
                    return false;
                } else {
                    return true;
                }
            });
            
            let size_val: Option<i32>;

            if size == "" {
                size_val = None;
            } else {
                size_val = Some(size.parse::<i32>().unwrap());
            }

            Ok(ApkProgram {
                name: program_name.to_string(),
                long_name,
                version,
                description,
                website,
                size: size_val,
                update: update_info,
                sub_versions: subversions
            })
        }
        Err(error) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, error))
    }
}

pub fn list_all_apk_programs() -> std::result::Result<Vec<ApkProgram>, std::io::Error> {
    let get_all_apk_program_names = std::process::Command::new("apk")
                                                                            .arg("info")
                                                                            .output();

    match get_all_apk_program_names {
        Ok(output) => {
            let get_output = std::str::from_utf8(&output.stdout).unwrap();

            let mut all_apk_programs = vec![];

            for line in get_output.lines() {
                let apk_program = get_apk_program(line.trim()).unwrap();

                all_apk_programs.push(apk_program);   
            }

            Ok(all_apk_programs)
        },
        Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error))
    }
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