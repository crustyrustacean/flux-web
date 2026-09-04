// src/main.rs

// 1. **What do I have?** State it literally, as a fact. Data, a variable, a file, a socket, nothing. It can't be wrong.
// 2. **What's the one next thing that has to be true?** Not the goal. One thing, closer to it.
// 3. **Can I do that by hand on a tiny input?** Yes → do it, then translate. No → too big; split it, back to 2.
// **Stuck means shrink, not close.** Blank at any question is a signal the step is wrong-sized, not that you can't.
// **Two kinds of stuck:** *I don't know what* → shrink. *I don't know how* → look it up (docs, examples first). Name which one before doing anything.
// **Run it, don't judge it.** Trace on paper, compiler, print, debugger — any cheap judge beats the one in your head.

use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // make a listener
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    for stream in listener.incoming() {
        match stream {
            Ok(_) => println!("We got one!"),
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}
