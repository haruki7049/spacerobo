# Spacerobo manual

## Configuration file

The configuration file is a TOML file. The values below are the defaults that Spacerobo uses when no configuration file
can be loaded.

```toml
[player.keyboard]
forward = "KeyW"
back = "KeyS"
left = "KeyA"
right = "KeyD"
dash = "ShiftLeft"
hover = "ControlLeft"
toggle_firemode = "KeyT"
quit = "Escape"
respawn = "Space"

[player.mouse]
x_reverse = false
y_reverse = false

[player.robo.thruster.force]
accelerate = 0.7
dash = 3.0
pitch = 1.0
yaw = 1.0
roll = 1.0
```

Key names are Bevy's `KeyCode` variant names, for example `"KeyW"`, `"ControlLeft"` or `"Escape"`.

### player.keyboard

Player's key configs.

#### player.keyboard.forward

Moves forward when you pressed the key. Default: `"KeyW"`.

#### player.keyboard.back

Moves back when you pressed the key. Default: `"KeyS"`.

#### player.keyboard.left

Moves left when you pressed the key. Default: `"KeyA"`.

#### player.keyboard.right

Moves right when you pressed the key. Default: `"KeyD"`.

#### player.keyboard.dash

Dash key. Hold it together with one of the movement keys (`forward`, `back`, `left` or `right`) to move with the
`player.robo.thruster.force.dash` force instead of the `accelerate` force. Default: `"ShiftLeft"`.

#### player.keyboard.hover

Hover key. You might want to use hover if your viewpoints are so intensely mixed up that you are not sure which direction you are looking in. While the key is held, your movement and rotation slow down. Default: `"ControlLeft"`.

#### player.keyboard.toggle_firemode

Toggle firemode key. The default mode is full auto. Use this key if you want to toggle full auto and semi auto. Default: `"KeyT"`.

Note: the game currently reads the `T` key directly and ignores this setting.

#### player.keyboard.quit

Quit key. Returns to the title screen. Default: `"Escape"`.

#### player.keyboard.respawn

Respawn key. Respawns your robo after it has been destroyed. Default: `"Space"`.

### player.mouse

Player's mouse configs.

#### player.mouse.x_reverse

Reverses the horizontal mouse axis. Default: `false`.

#### player.mouse.y_reverse

Reverses the vertical mouse axis. Default: `false`.

### player.robo.thruster.force

The force of the robo's thrusters. A larger value makes the robo accelerate or rotate faster.

#### player.robo.thruster.force.accelerate

Force of the movement keys (`forward`, `back`, `left` and `right`). Default: `0.7`.

#### player.robo.thruster.force.dash

Force of the movement keys while the `dash` key is held. Default: `3.0`.

#### player.robo.thruster.force.pitch

Force of the mouse rotation about the pitch axis. Default: `1.0`.

#### player.robo.thruster.force.yaw

Force of the mouse rotation about the yaw axis. Default: `1.0`.

#### player.robo.thruster.force.roll

Force of the mouse rotation about the roll axis. Default: `1.0`.
