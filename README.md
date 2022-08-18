# CocoaBird
A proof of concept bevy WinitPlugin replacement for mac

DONT't use for production

run a test using `cargo test -q main -- --nocapture`

has been done:
- replaced winit window creation
- replaced winit event loop

has to be done:
- fix window events
- re implement input
- add window resizing etc..
- profile a possible 2nd blocking proccess in the bevy source 
