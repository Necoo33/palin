# Palin

## v0.8.0

`AptSourceRepo` struct and `add_source_repo_to_apt` functions added. That benefits you to add a source repo to apt package manager with options, release, parts and comments.

## v0.7.0

`list_all_yum_programs()` function updated, now it not returns an empty yum program on the 0. index. Also i discovered that function's don't work properly if your system's language is not english, so use it with caution. Appeared that the recent bugs happened because my fedora and centos system language is not english. In next releases i'll add turkish support for that function, about adding support for other language for that function, your contributions will be welcome.

## v0.6.0

`list_all_yum_programs()` and `get_yum_program()` functions updated, now they also work if your os don't use yum but use dnf.

## v0.5.0

`find_package_managers()` function updated, now it detects apk package manager.
Support for apk extended. Added `ApkProgram` and `ApkProgramSubVersion` structs and `check_if_exist_in_apk()`, `list_all_apk_programs()` and `get_apk_program()` functions.

## v0.4.0

Support for pacman extended. Added `PacmanProgram` struct and `list_all_pacman_programs()`, `get_pacman_program()`.

## v0.3.0

Support for yum extended. Added `YumProgram` struct and `list_all_yum_programs()`, `get_yum_program()`.

## v0.2.0

Support for apt extended. Added `AptProgram` struct and `list_all_apt_programs()` and `get_apt_program()`.

## v0.1.0

Palin Released.


