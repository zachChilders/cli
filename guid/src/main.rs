use uuid::Uuid;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let uuid = Uuid::new_v4();
    println!("{}", uuid);
    ctx.set_contents(uuid.to_string()).unwrap();
}
