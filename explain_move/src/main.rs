struct User {
    name: String,
}

fn some_action_to_user(user: &User) {
    let tmp = user;
    // 何かアクションをする
    println!("{}", tmp.name);
}

fn main() {
    let user = User {
        name: String::from("user_a"),
    };
    // 関数に user を渡すと、所有権がこの関数に移ってしまう。
    some_action_to_user(&user);
    // なので、この時点では user はもう main 関数には所有権がなくなっている。
    // したがってコンパイルエラーとなる。
    println!("one more: {}", user.name);
}
