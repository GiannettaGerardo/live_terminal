# live_terminal
A rust program to update the terminal in real time


## Usage

```bash
# directory_path must be a valid path to a directory in the filesystem
./live_terminal directory_path
```

## How does it work

The program is a simple Rust version of the "ls -la" command, but in addition it listens for any changes in the reported directory (as an argument) and updates the list on the screen in real time.

After running the program in your terminal and passing it a directory as an argument, try looking at the terminal and in the meantime try changing that directory. You will see the changes made update in real time.

## License
[MIT](https://choosealicense.com/licenses/mit/)
