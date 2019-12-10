// We have a script that generates a bunch of executable steps.
use std::path::Path;

mod traits;
mod users;

#[derive(Debug)]
struct LocalFile<'a> {
    path: &'a Path,
    permissions: String,
    contents: String,
}

impl<'a> traits::Step for LocalFile<'a> {
    fn apply(&self) -> Result<(), ()> {
         Ok(())
    }
    fn dry_apply(&self) -> Result<(), ()> {
        Ok(())
    }
    fn delete(&self) -> Result<(), ()> {
        self.path.is_file();
        Ok(())
    }
}



fn make_plan(crud: Vec<&dyn traits::Step>) {
    for x in crud {
        println!("{:?}", x);
        x.apply();
    }
}

fn main() {
    let lf = LocalFile {
        path: &Path::new("/etc/nix/nix.conf"),
        permissions: "-rwxr--r--".to_string(),
        contents: include_str!("default-nix-conf").to_string(),
    };
    let lf_uid = users::Users {
        n_users: 16,
        gid: 30000,
        name: "nixbld".to_string(),
    };
    let desired: Vec<&dyn traits::Step> = vec![
        &lf,
        &lf_uid,
    ];
    let plan = make_plan(desired);
    let gid = 30000;

}
