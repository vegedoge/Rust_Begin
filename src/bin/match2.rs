enum Action
{
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main()
{
    let actions = [
        Action::Say("Hello, Rust!".to_string()),
        Action::MoveTo(1, 3),
        Action::ChangeColorRGB(255, 0, 255),    
    ];
    for action in actions
    {
        match action
        {
            Action::Say(s) => 
            {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => 
            {
                println!("moved to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, b) =>
            {
                println!("RGB changed to ({}, {}, {})", r, g, b);
            }
        }
    }
}