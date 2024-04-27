# Tagger

**Tagger is a super lightweight navigation utility based on registers and written in Rust.** Tag directories that you want to revisit and return to them later by moving into the specific register. This util comes in handy when you find yourself `cd`-ing between the same directories a lot. 

## Build
Clone the repo and run `cargo build --release` to build it. Add the binary to your `$PATH`.

*This tool uses nerdfont to render glyphs in the status messages. For the best experience, make sure that you have a proper font installed in your terminal.*

## Setup
Tagger requires minimal shell configuration to get going. Add the following to your shell configuration file of choice like `.bashrc` or `.zshrc`:

```bash
# tagger
alias list="source <(tagger list)"
alias purge="source <(tagger purge)"
tag() {
  source <(tagger tag $@)
}
move() {
  source <(tagger move $@)
}

```
This process substitution is necessary so that the working directory of your current shell session as well as the stack's env var can be overwritten. Of course you can replace the alias and function names with names you like.

## Usage

Tag the current directory and add it to the next empty register:
```console
foo@bar:~/some/directory$ tag
Pushing /home/user/some/directory into empty register 0
```

Move into the last register's directory:
```console
foo@bar:~/some/directory$ cd ~
# do some work in the home directory...
foo@bar:~$ move 
Moving into register 0: /home/user/some/directory
foo@bar:~/some/directory$
```

Print all registers:
```console
foo@bar:~/some/directory$ cd nested/very/deeply && tag
Pushing /home/user/some/directory/nested/very/deeply into empty register 1
foo@bar:~/some/directory/nested/very/deeply$ list
Current registers: 
[0] /home/user/some/directory
[1] /home/user/some/directory/nested/very/deeply
```
Purge all registers: 
```console
foo@bar:~/some/directory/nested/very/deeply$ purge
Purging registers 
foo@bar:~/some/directory/nested/very/deeply$ list
Current registers are empty
```

You can **move into and tag specific registers** by providing an index as the second argument:

```console
foo@bar:~$ list 
[0] /home/user/directory/A
[1] /home/user/directory/B
[2] /home/user/directory/C
[3] /home/user/directory/D
foo@bar:~$ move 3 
Moving into register 3: /home/user/directory/D
foo@bar:~/directory/D$ cd baz && tag 8
Pushing into empty register 8: /home/user/directory/D/baz
foo@bar:~/directory/D/baz$ list
[0] /home/user/directory/A
[1] /home/user/directory/B
[2] /home/user/directory/C
[3] /home/user/directory/D
[8] /home/user/directory/D/baz
```

If the register is not empty then the contents are overwritten:
```console
foo@bar:~/directory/D/baz$ cd ~/somewhere/else && tag 8 && list
Overwriting register 8: /home/user/somewhere/else
Current registers: 
[0] /home/user/directory/A
[1] /home/user/directory/B
[2] /home/user/directory/C
[3] /home/user/directory/D
[8] /home/user/somewhere/else
```
If all registers are full and the index is omitted when tagging a new directory, then the last register will be overwritten (works like a stack). 
