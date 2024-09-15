# Palin - An utility liblary for package managers.

Palin is a Utility Liblary for linux's package managers, such as apt, yum, pacman. It can detect which package managers installed in a linux distro, checks if a program installed in that. In next releases, new utilites will be added.

Consider to give a like on [github repo](https://github.com/Necoo33/palin) if you like that crate.

## Guide

```rust

use palin::*;

fn main() {
    let package_managers = find_package_managers(); // this returns: Vec<&'a str>

    // then check which package managers exist and do your stuff depending on that. It usually will return this kind of answer:

    // ["apt", "dpkg"]
    // ["yum", "dnf", "rpm"]
    // ["apk", "busybox"]
    // etc...
}

```

Then you can check is a program installed on a package managers list. That functions returns a boolean:

```rust

use palin::*;

fn main(){
    let is_wget_exist_in_apt = check_if_exist_in_apt("wget");
    let is_wget_exist_in_dpkg = check_if_exist_in_dpkg("wget");
    let is_wget_exist_in_yum = check_if_exist_in_yum("wget");
    let is_wget_exist_in_dnf = check_if_exist_in_dnf("wget");
    let is_wget_exist_in_rpm = check_if_exist_in_rpm("wget");
    let is_wget_exist_in_pacman = check_if_exist_in_pacman("wget");
    let is_wget_exist_in_busybox = check_if_exist_in_busybox("wget");
}

```

And also you can check existences of some specific programs:

```rust

use palin::*;

fn main(){
    let is_curl_exist = check_if_curl_exist();
    let is_wget_exist = check_if_wget_exist();
    let is_dig_exist = check_if_dig_exist();
    let is_ip_exist = check_if_ip_exist();
}

```

Then you can get an apt program if apt exist:

```rust

use palin::*;

fn main(){
    // check if apt exist before

    let get_wget_program = get_apt_program("wget").unwrap();

    // do your other stuff 
}

```

## Planned Features For Future Releases

* Adding support for listing the programs of package managers.
* Adding support for showing more information support for programs of package managers.
