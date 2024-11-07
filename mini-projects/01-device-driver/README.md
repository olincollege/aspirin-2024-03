# Mini Project 1: Device Driver
Hello new developer! My name is Ayush Chakraborty and I am the CEO of Rustbox Studios. Myself, along with the rest of the team, are very excited to have you on board to help make our products safer! We recently suffered a massive outage due to some forgetfulness (memory unsafety) in our old code that we can't C anymore. As a result, we decided to migrate our entire codebase to rust (ironic that it wasn't already in rust huh?). Luckily for you, you are not alone in this endeavor. I have already migrated the initial state machine running on the game controller and am here if you need any more resources. We have also tasked another developer (your partner for the project) to assist you.

## 0. Setup

### 0.1 Sync and Branch your Repo

Make sure that you are at the latest change in your repo by running the following commands:

```
$ git switch main
$ git pull
$ git pull upstream main
$ git push
```
If you run into issues performing the above operations, ask for help on Discord.

Once you have done this, create a new branch for this assignment:
```
git switch -c mini-project-01
```
### 0.2 Read the Rubric

The rubric is given to you to provide you an idea of how your work will be assessed, what is expected of you, and what you can expect to get feedback on. You can complete the assignment without reading it, but we strongly recommend that you read it to get a sense of how assignments are graded.

## 1. Technical Guide
Before jumping in to your tasks, let's take a brief tour of the infrastructure you have at your disposal.
### 1.0. Hardware Overview
The primary piece of hardware produced at our company are our game controllers. The base model is powered by a [Raspberry Pi Pico W](https://www.raspberrypi.com/products/raspberry-pi-pico/). Connected through a solderless breadboard (don't ask why our production models still use breadboards...), there are 3 LEDs: a red "Ready" LED connected to GPIO18, a yellow "Set" LED connected to GPIO17, and a green "Go" LED connected to GPIO 16. Additionally there are four buttons that allows users to move characters. THe buttons represent each of the 4 orthogonal directions (northeast: GPIO13, northwest: GPIO12, southeast: GPIO15, southwest: GPIO14)

For your reference, the full Raspberry Pico pinout can be found [here](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html)

Additionally, you can find the full schematic of our board (in case any connections come loose) in the `pico-hardware` folder.

### 1.1. Software Overview
Now onto the software that I have already implemented on the raspberry pico. In short, I have implemented a simple state machine that exposes a serial interface to initialize the device, drive the LEDs, and obtain the state of each button when appropriate. 

The firmware was written with the [rp-pico](https://crates.io/crates/rp-pico) crate that serves as our *Hardware Abstraction Layer*. Ayush will be giving a more in-depth look at how the firmware works (at a low level) on 11/11 for those interested. All of the firmware can be found in the `pico-image` package.

The following outlines the state machine running on the raspberry pico (the various states are captued by the DeviceState enum):
1. `DeviceState::PendingInit`: When the device is first powered, it starts in the `DeviceState::PendingInit` state. In this state, the device will not respond to any commands, nor will it send any data over serial. When "init controller" is received via serial, the device transitions to `DeviceState::PendingStart`.
2. `DeviceState::PendingStart`: This state refers to the device already running any necessary initialization routines, but the commander has not requested a game to start playing. This is the only state where the "Ready", "Set", and "Go" LEDs may be commanded to turn on/off. Associated Commands (via serial):
    - "set ready led": This turns on the red LED and turns off all other LEDs.
    - "set set led": This turns on the yellow LED and turns off all other LEDs.
    - "set go led": This turns on the green LED and turns off all other LEDs.
    - "set all LEDs": This turns on all three LEDs.
    - "clear all LEDs": This turns off all three LEDs.

    From this state, the device can be transitioned to `DeviceState::Running` by sending "stop controller" via serial.

3. `DeviceState::Running` signifies that a game is actually being played. This means that the commander needs to know the current state of each button so that it can update the position of each player. That means that within this state, the controller sends the state of each button (as a bitmask) over serial. The value sent over is mapped as following (most significant bit to least significant bit):
    - Bit 3: The state of the Northwest button.
    - Bit 2: The state of the Southwest button.
    - Bit 1: The state of the Southeast button.
    - Bit 0: The state of the Northeast button.
    From this state, the device can be transitions to `DeviceState::Complete` by sending "stop controller".
4. `DeviceState::Complete` represents the end of a game round. The device takes no actions from this state, but can be transitioned to any of the other three states to prepare for another game round. Those transitions can be achieved by sending the following commands:
    - "reset" transitions the device to `DeviceState::PendingInit`
    - "restart" transitions the device to `DeviceState::PendingStart`
    - "start controller" transitions the device to `DeviceState::Running`

### 1.2. How to Flash Firmware
In order to flash new firmware to the raspberry pico, follow the following process.

1. Unplug the raspberry pico from your computer.
2. Press down on the "bootsel" button on the raspberry pico board.
3. Plug in the raspberry pico to your computer.
4. If done correctly, you should now see a `RPI-RP2` Volume mounted on your computer.
5. Run `cargo run` from the `pico-image` crate to start the flashing process.
6. Ensure no errors are outputted and then you're set!

## 2. Mini-Project Components
This section outlines the deliverables for the mini-project.

### 2.0. Onboarding Project
Here at Rustbox Studios, we take pride in supporting new hires to the best of our abilities. For that reason, we require all employees go through a thorough onboarding project to get to know our full stack, while also making meaningful contributions. *You are required to complete all parts of the onboarding project for this assignment*

At a high level, I would like you to design and develop a laptop interface that simultaneously integrates two controllers with a simple GUI (can just be a terminal interface) to track the position of each controller.

The key subcomponents of this task are as follows:

1. Serial Interface: In order to communicate to the game controllers, you need to be able to send and receive serial data. I would like you to use the [`libserialport` library](http://sigrok.org/wiki/Libserialport) library to manage all low-level serial commands through Rust's FFI. 
2. Parallel Processing: I would like you to prioritize minimizing processing latency from the game controller when updating the position of the player. In order to do this, I recommend you dedicate a thread for each game controller in order to call blocking read commands on the respective serial ports, and then displaying their positions in a third thread.
3. Controller Management: The laptop interface should be able to step through both game controller's state machines to prepare a round of the game. During the `DeviceState::PendingStart` state, the on-board LEDs should be turned on/off with a corresponding countdown to signify to the users that the game is about to start. Additionally, it should be able to reset/restart the controller so that consecutive rounds can be played without power-cycling the controller.
4. Game State Management: Within the backend of your game interface, you should be able to manage the state of the game. This means having a state machine that transitions between prompting the user the instructions for the game, starting the game, announcing the winner, and starting another round. Additionally, you need to be able to track the position of each game controller when the game is running. One potential way to define "tracking the position of the game controller" is:
    - Every time a button is pressed, the player moves 1 unit horizontally and vertically towards the direction of the button. For example, if a player is at (0, 0) in (x, y) and the Northeast Button is pressed, they will now be at (1, 1).
    - Additionally, the players movement will only be updated on the rising edge of a button press. This means that if 2+ consecutive messages say that a specific button is being pressed, the position of the player will only be updated once.
5. User-Interface: Finally, there should be a simple interface that allows users to play a game and interface with their controllers. This can be as simple as periodic prints to the terminal, to as complex as a full graphical user interface. No matter the format, the interface needs to have the following components.
    - Notifies the user that the controllers are connected.
    - Allows the user to start a game round.
    - Runs through a basic game with the controllers (I'm giving you the creative freedom to determine the logistics of the game!).
    - Output which player has won the game. 
    - Allows users to start another round.
    - Allows the users to end the game and shutdown the controllers.

In the end, the game flow should be able to look something like this (NOTE: You do not need to conform to the specifics of this terminal output, it just serves as an example):
```bash
$ cargo run
Controller 0 Initialized.
Controller 1 Initialized.
Press Enter to Start the Game: \n
Starting in 3. (RED LED turns on)
2. (YELLOW LED turns on)
1. (GREEN LED turns on)
GO (all LEDs turn on)
Player 0 Position: (0, 0)   Player 1 Position (0, 0)
Player 0 Position: (1, 0)   Player 1 Position (0, 1)
Player 0 Position: (2, 0)   Player 1 Position (0, 2)
Player 0 Position: (3, 0)   Player 1 Position (0, 1)
Player 0 Position: (4, 0)   Player 1 Position (0, 0)
<space bar pressed>
The game has concluded! Player 0 traveled the furthest and is the winner!
Press Enter to Start Another Round: 
```

### 2.1. First Sprint
Now that both of you are fully up to speed with our stack, we would like you to contribute to our next sprint, which ends on 11/21. As a team, we use a ticket-based system where each developer is free to take on any outstanding tickets. Each ticket has been assigned a number of story points to estimate the amount of effort I expect the task to require. While I don't have a specific preference about which tasks you take on, I am requiring that you complete 10 story points worth of tasks (extra credit on the assignment if you complete more). 

Also, I encourage you, as you examine our stack, to find your own sources of improvement and create your own tickets where you see fit. (Please reach out to Amit or Ayush about any potential tasks you want to take on so that they can access whether the task aligns with the goals of the company.) I do not want you to feel bottlenecked by the existing tasks and highly encourage you to think of tasks beyond this list! Here is the current list of outstanding tasks:

Tasks to improve the Game Controller Firmware:
- (3 Story Points) Add more buttons. Currently, the 4-button interface severely limits the movement of users. To improve the hardware, I suggest modifying the board to have 8 buttons, maintaining the current diagonal movements, but also adding orthogonal movements. This will require re-wiring the board (I have bigger breadboards to assist with this) and adding in 4 more digital read pins that need to have their state broadcasted over serial.
- (3 Story Points) From my user studies, I have learned that a lot of users feel limited by the 1 unit movements of each button press. As a potential solution, I suggest that we add a potentiometer to the board, where each player's movement can be scaled based on the current reading of the potentiometer. This will require adding the circuitry for the potentiometer to the current game controller, utilizing the ADC on the RP2040 to read the state of the potentiometer, and broadcasting this reading over serial.
- (2 Story Points) Currently, there is a severe limitations in how our system processes the button statuses. The firmware is designed to only transmit the state of the buttons every 100 ms, and only checks the state of the button when it is about to transmit. This means that a button press could be missed if it is not actually being actuated during that polling. To improve this, I suggest using interrupts to continuously track the state of the led, and then use that state for transmission(definitely reach out to Ayush for more details about this).
- (2 Story Points) Currently, all the compute related to the position of the controller is done on the laptop, but wouldn't it be cool if this could be done directly on the controller? This would better modularize the software stack, so that the laptop just needs to take in the current position of the controller (sent directly over serial) and be reset when appropriate.

Tasks to improve the Laptop Interface:
- (2 Story Points) For the onboarding project, you only have to handle 2 controllers. Now, we want you to adopt your serial interface (and game logistics) for any arbitrary number of controllers.
- (1-4 Story Points) This bullet is fully up to you! Wouldn't it be really cool of the game we are playing is more than just tracking the position of the controller? This task gives u creative liberty to add any game logic that you want. Make sure to reach out to the teaching team to agree on the number of story points (scope) of your game.
- (3 Story Points) Some of our users aren't the biggest fans of our terminal-based interface. One improvement that could be made is adding an actual GUI to run our game. I highly recommend using one of rust's crates that make generating GUIs very easy! One example is [egui](https://docs.rs/egui/latest/egui/). 
- (3 Story Points) Game Metrics! Currently, our base game only displays who won the game, but doesn't provide any information about a player's performance! I believe adding visualizations of the player's position over time could be really helpful! Rust's [plotters](https://docs.rs/plotters/latest/plotters/) library could be helpful for you to render this plot! I would also like you to add another plot of your choose to display something that you believe is critical for players to understand how well they did in your game!

## 3. Submission
To submit this assignment, add and commit your changed files (don't forget to format and lint!). These should just be the files in the src directory. Be sure to write a reasonably clear commit message.

Once you have committed your changes, push them to origin (your fork of the course repository) and open a pull request to your main branch. Assign the team "olincollege/aspirin-2024-03-assistant" as reviewers.