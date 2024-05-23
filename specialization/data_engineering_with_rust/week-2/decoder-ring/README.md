# Decoder ring

## Improvements

- Implemented feature that allows the user to input a message through a file rather than a command-line argument;

- Optimized the scoring mechanism in `guess_shift`.

## Running

Run the following command in order to guess a message passed as a command-line argument:

```bash
cargo run -- --message "Your encrypted message here" --guess
```

Run the following command in order to guess a message passed through a file:

```bash
cargo run -- --file message.txt --guess
```