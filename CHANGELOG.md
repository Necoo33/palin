# Palin

## v0.6.0

`list_all_yum_programs()` and `get_yum_program()` functions updated, now they also work if your os don't use yum but use dnf. Also i noticed that functions are only working at wsl's, not working on centos 9 or fedora 39, because they don't return the required terminal command directly and we are working on another solution for that.

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


