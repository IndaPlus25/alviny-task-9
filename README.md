# alviny-task-9: az09

This is alviny's 8th weekly INDA task: the az09 assembly language.

See specifications.md for more information about the language, including instruction set and syntax. Example programs can also be found in `sample.az09`, `helloworld.az09`. The factorial program requested in the assignment is in `factorial.az09`.

## How to run an az09 program (Windows)

1. Download the az09 executable `az09.exe` by 1) `git clone`ing the repository, or alternatively 2) clicking on the file name in Github, then clicking on "more file actions" (three dots in a gray square to the top right) and choosing "download".
2. In the command line, do the following (Windows):

```bash
cd alviny-task-9 (or wherever you decided to download the file)
./az09 "path/to/your/az09/program.az09"
```

Of course, replace `"path/to/your/az09/program.az09"` with the actual path to the az09 program you are trying to run.

## How to run an az09 program (Non-Windows):

You will have to build your program yourself.

To build (with Cargo):

```bash
cargo build --release
```
Then you will find a new directory with your EXE, ready to run.

