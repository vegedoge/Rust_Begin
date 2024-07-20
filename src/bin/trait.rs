// Trait
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Post{
    fn summarize(&self) -> String{
        format!("Post is {}, author is {}", self.title, self.author)
    }
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("Weibo user is {}, content is {}", self.username, self.content)
    }
}

fn main(){
    let post = Post{title: "My post".to_string(), author: String::from("Blake"), content: "nonconvex".to_string()};
    let weibo = Weibo {username: String::from("melon"), content: "my weibo".to_string()};
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}