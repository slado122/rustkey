use clipboard::{ClipboardContext, ClipboardProvider};

pub fn put_to_clipboard(string: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(string.to_owned()).unwrap();
}
