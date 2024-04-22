use clap::{command, Arg, ArgGroup};
fn main(){

    let match_res = command!()
        .about("Hi im just learning")
        .group(ArgGroup::new("person").arg("firstname").arg("lastname"))
        .group(ArgGroup::new("dawg").arg("dogname").arg("happy"))
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname","firstname","first-name"])
                .required(true)
                .help("Persons first name"))
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname","lastname","last-name"])
                .required(true)
                .help("Persons last name"))
        .arg(
            Arg::new("dogname")
                .short('d')
                .help("Define dogs name 🐕🐶")
        )
        .arg(
            Arg::new("happy")
                .short('H')
                .help("Define if the person is happy or not 🤣🤣🤣🫵")
        )
        .get_matches();
    println!("{:?}",match_res.get_raw("firstname").unwrap());

}