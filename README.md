# Bounding Box

A rust program that finds the largest minimum bounding box from an input diagram containing either '-' or '*' characters.

## Run the Program:

Install rust with rustup: https://www.rust-lang.org/tools/install

Then either pass in a text file like:

'''bash
$ cargo run < groups.txt
'''

or write to stdin, separating input with newlines. To finish data input, enter an empty newline.

'''bash
$ cargo run
$ --*-
$ --**
$
'''

### Using Docker:

Install Docker on your machine: https://docs.docker.com/engine/install/
As above, you can either run with a text file or enter in text via stdin.

'''bash
$ docker build -t boxes .
$ docker run -i boxes < groups.txt
'''
