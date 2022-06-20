# live_terminal
A simple bash script to update the terminal in real time


## Usage

```bash
# directory_path must be a valid path to a directory in the filesystem
./live_termina.sh directory_path
```

## How does it work

The program uses the bash ls -la command to print the complete list of all files present in a directory passed as a script parameter.

After starting the program, the ".f.txt" file is created in the directory where live_server.sh is located. This file is used to perform the main check to update the terminal in real time. Do not delete the file.

After starting the script, it points to the fileststem directory passed as the script argument. Then try to start a new terminal in the same directory that the script is pointing to. Try to make changes in that directory and in the meantime look at the script on the other terminal. You will see the changes made update in realtime.

## License
[MIT](https://choosealicense.com/licenses/mit/)
