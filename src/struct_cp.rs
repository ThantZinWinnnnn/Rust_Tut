

pub fn struct_eg () -> () {
    struct Anything;//unit struct

    struct User {
        active: bool,
        username : String,
        email : String,
        sign_in_count : u64,
    }

    fn build_user(email:String,username:String)-> User {
        User {
            active : true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user1 = build_user(String::from("thantzinwin@"), String::from("thantzinwin"));

    let user2 = User {
        email : String::from("thanzawwin@"),
        ..user1
    };

    //better way for using struct
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    };
    
    fn area (rectangel: &Rectangle) -> u32 {
        rectangel.width * rectangel.height
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {:#?}",rect1)
}

