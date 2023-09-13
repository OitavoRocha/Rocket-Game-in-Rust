# Rocket Game in Rust
 A simple rocket game written in rust language, utilizing the GGEZ game library

## Game Features and Rules
    - Inputs are the planet's gravity and the spaceship's weight and fuel;
    - The goal is the land on the platform correctly;
    - "Up" and "Down" control the motor's potency;
    - "Left" and "Right" control the spaceship's movement, being able to move and adjust mid-air;
    - If theres no fuel left, the spaceship crashes in to the ground and you loose;
    - If the spaceship is too fast, you also crash and loose;
    - To win, you need to land at a low enough speed and not loose track of your fuel usage.

## Running the game
Use the following command line to run the game:
` cargo run (fuel in float) (gravity in float) (weight in float)`
Just change the in parentheses parts to actual numbers that represent your spaceship's status.
