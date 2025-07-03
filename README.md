# d6-rs

tiny, lightweight diceroll program
made this cuz i found out dice symbols exist

## installation and usage
clone this repo
```
git clone https://github.com/ellipticobj/d6-rs.git && cd d6-rs
```

install
```
cargo install --path .
```

run
```
d6
```

## configuration
the config file should be located at ~/.config/d6.cfg

each line in the config file should look like this:
```
token: value
```
note: whitespace doesnt matter

### dicesize
changes the default number of faces of the die.

```
dicesize: <integer>
```

### animation
activates or deactivates the dice rolling animation.
```
animation: <true|false>
```

### animdur
changes duration of the animation (in seconds).
```
animdur: <float>
```

---

made with rust btw haha

## plans !
- [ ] add more robust checks for custom dice sizes
- [ ] finish cofiguration for faces, animation frame intervals
- [ ] add color?
- [x] customization? (animation time, output options, symbols)
- [x] more dice options
- [x] add a way to pipe just the value or to remove the symbol so bash scripting is easier
