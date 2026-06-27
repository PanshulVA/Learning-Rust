fn main() {
    // struct User {
    //     active: bool,
    //     username: String::from("Panshul28"),
    //     email: String::from("aggarwal.panshul.vikram@gmail.com"),
    //     sign_in_count: u64,
    // }
    let mut user1= struct User {
        active: bool,
        username: String::from("Panshul28"),
        email: String::from("aggarwal.panshul.vikram@gmail.com"),
        sign_in_count: u64,
    };
    user1.email = String::from("panshul.vikram.aggarwal@outlook.com") //In a mutable struct change stuff using dot(.)


    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    //OR
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 //All other fields not specified to be taken from user 1 , must come last to specify that any remaining fields should get their values from the corresponding fields
    };// User 1 is not valid  when user 2 comes in scope since String does not show copy trait, If we had specified username, then user1 would not need to be invalid.
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, //Tedious to write username and email again and again
        email: email, //Tedious to write username and email again and again
        sign_in_count: 1,
    }
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //Better way.
        email, //Better way.
        sign_in_count: 1,
    }
}


