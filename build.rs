extern crate git2;
use git2::{ObjectType, Repository};

extern crate chrono;
use chrono::offset::Utc;

fn main() {
    // Commit hash
    let repo = Repository::open(".").expect("Couldn't open git repository");
    let head = repo.head().expect("So no HEAD? *Slams phone on ground and snaps skateboard in half*")
        .resolve().expect("Couldn't resolve reference to HEAD");
    let commit = head.peel(ObjectType::Commit).expect("Couldn't find commit at HEAD")
        .into_commit().expect("HEAD reference was not a commit");

    let hash = &format!("{}", commit.id())[..9];

    println!("cargo:rustc-env=GIT_HASH={}", hash);

    // Build date
    let date = Utc::now().date().naive_utc();

    println!("cargo:rustc-env=BUILD_DATE={}", date);
}