# Tagger

**Tagger is a super lightweight, stack-based navigation utility written in Rust.** Tag directories that you want to revisit and return to them later by popping them off the stack. 

## Setup
Tagger requires minimal shell configuration to get going. Add the following 2 lines to your shell configuration file of choice like `.bashrc` or `.zshrc`:

```bash
alias tag="source <(tagger tag)"
alias pop="source <(tagger pop)"
```
This proccess substitution is necessary so that the working directory of your current shell session as well as the stack's env var can be overwritten. Of course you can replace the alias with a name you like.

## Usage

Tag the current directory:
```console
foo@bar:~/some/directory$ tag
Tagged /home/some/directory
```

Pop off the last directory and automatically cd into it:
```console
foo@bar:~$ pop
Popped /home/some/directory
foo@bar:~/some/directory$
```
