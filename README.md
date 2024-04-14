# Tagger

**Tagger is a super lightweight, stack-based navigation utility written in Rust.** Tag directories that you want to revisit and return to them later by popping them off the stack. This util comes in handy when you find yourself `cd`-ing between the same directories a lot. 

## Build
Clone the repo and run `cargo build --release` to build it. Add the binary to your `$PATH`.

*This tool uses nerdfont to render glyphs in the status messages. For the best experience, make sure that you have a proper font installed in your temrinal.*

## Setup
Tagger requires minimal shell configuration to get going. Add the following 4 lines to your shell configuration file of choice like `.bashrc` or `.zshrc`:

```bash
alias tag="source <(tagger tag)"
alias pop="source <(tagger pop)"
alias list="source <(tagger list)"
alias purge="source <(tagger purge)"
```
This process substitution is necessary so that the working directory of your current shell session as well as the stack's env var can be overwritten. Of course you can replace the alias with a name you like.

## Usage

Tag the current directory:
```console
foo@bar:~/some/directory$ tag
Tagged: /home/some/directory
```

Pop off the last directory and automatically `cd` into it:
```console
foo@bar:~$ pop
Popped: /home/some/directory
foo@bar:~/some/directory$
```

Print the stack:
```console
foo@bar:~$ list
Current stack: 
/home/some/directory
/home/some/directory/nested/very/deeply
```
Purge the stack (remove all entries): 
```console
foo@bar:~$ purge
Purged stack
foo@bar:~$ list
Current stack is empty
```
