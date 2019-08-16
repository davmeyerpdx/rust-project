# Virtual Blackjack
**CS410: Rust Programming (Summer Term)**
***by Wei Cao & David Meyer***

## Description
*This game allows a user to play blackjack against the computer. Starting with a balance of 100 chips, the user can play as long as they want either until they quit or lose all their chips. It follows all the normal conventions of blackjack and and even provides an intuitive GUI for ease of use.*

## How to run
*This game was built using the ggez framework. As it stands now, all you must do to run this program is to 
```
cargo run
```
Playing the game, however, cannot be accomplished through ssh such as putty. As a disclaimer, when trying to run this game on a fresh PC we encountered a few difficulties and ended up having to download several packages before we were able to start. These packages were prompted to us. We hope that you do not encounter the same issue.*

## An Example Illustrates the Operation of the Code
```
use ggez::*;
//use gges::...;
mod Model{
  struct Card;
  struct Deck;
  
  trait Player;
  struct Banker;
  struct Human;
}
struct MainState{
  //...
}
impl MainState{
  fn new(ctx: &mut Context) -> GameResult<MainState> {
    //...
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult{
      //...
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
      //...
  }
  fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
      //...
  }
}
pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("game_name", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}

```
## Testing
*The test file is located in the following path "src/lib/model/tests.rs." These are somewhat simple tests that ensure the functions in "player.rs" are implemented correctly. The tests run each function and ensures the expected output. In addition to this we also manually play-tested the game ourselves looking for areas for improvement.*

## Further Questions
*For questions or comments please email caowei@pdx.edu or davmeyer@pdx.edu*

