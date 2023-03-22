# gradient.rs 

A library for generating colour gradients in the command line 

## Usage 

``` rust
use gradient::{GradientBuilder, Colour};

let gradient = GradientBuilder::new()
    .text("Hello, World!")
    .start_colour(Colour::new(0x13, 0xE4, 0xEF))
    .end_colour(Colour::new(0x9B, 0x13, 0xEF))
    .bold()
    .italic()
    .build(); 
    
println!("{}", gradient);
```
