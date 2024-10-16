# mini-shell

# Assignment

# Matic - Systems Engineer Challenge

## Instructions:

**For clarity, the normally invisible newline character (ASCII 0x0A) is represented with **`<NL>`.**

Implement a simple POSIX/Unix shell (similar to bash).

The shell should support the following features at minimum:

* The shell should print "`$`" on standard error (stderr) as a prompt. Commands should be read from standard input (
  stdin), and are delimited by newline (ASCII 0x0A). Command tokens are delimited by one or more spaces, with the first
  token being the command to invoke and the other tokens being the arguments to pass to the command. Extra spaces before
  the first token and after the last token should also be stripped.
* The shell should be able to run commands by their absolute paths:

```
$ /bin/ls -l /usr/share<NL>
total 12
drwxr-xr-x 1 root root    20 Mar 27  2018 accountsservice
drwxr-xr-x 1 root root 26672 Jan  3 13:19 aclocal
drwxr-xr-x 1 root root   608 Dec 13 17:34 aclocal-1.16
drwxr-xr-x 1 root root    30 Apr 20  2020 acpi_call
...
```

* The shell should be able to run commands present in the environment variable `$PATH`:

```
$ cat /proc/mounts<NL>
proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
sys /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
...
```

* The shell should handle single quotes and double quotes. No backslash escape handling is required. If quotes are
  mismatched, print an error. Errors should be printed on standard error (stderr).

```
$ /usr/bin/printf "The cat's name is %s.\n" 'Theodore Roosevelt'<NL>
The cat's name is Theodore Roosevelt.
$ /usr/bin/printf "Missing quote<NL>
error: mismatched quotes
```

* The shell should handle the `cd` built-in command to change the current directory. It should take a single argument:
  the absolute or relative path of the directory to change into. If the command fails, it should print an error. Errors
  should be printed on standard error (stderr).

```
$ cd /sys<NL>
$ ls<NL>
block  bus  class  dev  devices  firmware  fs  hypervisor  kernel  module
power
$ cd /nonexistent<NL>
error: cd failed
```

* The shell should handle the `exit` built-in command to exit and return control to its caller.
* The shell should exit and return control to its caller when it receives end of file (EOF) on
  its standard input.
* If a command exits with a non-zero exit code, the shell should print the exit code. The exit code should be printed on
  standard error (stderr).

```
$ head /nonexistent<NL>
head: cannot open '/nonexistent' for reading: No such file or directory
error: command exited with code 1
```

* The shell must be written in C, C++, or Rust
* Do not use any external libraries.
* Notwithstanding out-of-memory errors or external signals, the shell should never crash, even if it receives invalid
  input. A crash is defined as the program exiting for any reason other than `main()` returning or `exit()` being
  called.
* The shell may optionally error if the command line is longer than 1000 characters (including the terminating newline).
  However, this should not crash the shell.
* The shell may optionally error if the command line contains more than 100 arguments. However, this should not crash
  the shell.
* The error messages may be formatted in any format of your choosing.
* Any POSIX shell details not specified in this specification should be omitted (e.g. signal handling, pipes,
  redirection, environment variable expansion, line editing, history, etc.)

# **Test cases**

Normal text is output from the shell itself on standard error (stderr).

**Bold **text is user input.
Red text is command output.

![](https://app.ashbyhq.com/api/images/user-content/78513062-19e3-4a0b-adbd-6326b08d2626/2dfe35c7-6545-47c1-80da-200f9f3b368e/Screen%20Shot%202023-01-09%20at%202.39.11%20PM.png)

## **Extra notes**

* In C/C++, child processes may be spawned using `fork()<span> </span>`and`<span> </span>execve(). posix_spawn()` is
  also an option. `wait() or waitpid()` may then be used to wait for the child process to exit.
* `execvp()<span> </span>`and `posix_spawnp()<span> </span>`additionally takecare of the \$PATH lookup for you.
* In Rust, std::process::Command may be used to spawn child processes

# Usage

```rust
cargo r
```

To exit type any of

- exit
- quit
- bye

Powered
by ](https://www.ashbyhq.com/)[Privacy Policy](https://www.ashbyhq.com/privacy)[Security](https://www.ashbyhq.com/security)[Vulnerability Disclosure](https://www.ashbyhq.com/disclosure)
