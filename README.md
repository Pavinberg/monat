# monat -- Modern file system Navigator

- [简体中文](./i18n/README-zh.md)

## Introduction

`monat` is a Unix shell auxiliary command focusing on the navigation of the file system, especially for working in big projects. Think of a scenario where you are at the root directory of a big project, and there are files lying in many deep directories. It's common to visit the files, rename or move them, run a command in some sub-directories and come back to the root, etc. It would be **tedious to input the long path prefix again and again**.

`monat` is designed to simplify this process by leveraging a simple syntax to express the long path prefix.

:warning: This project is still in the process of development. Your contributions are welcome.

## Example

![monat-demo](images/monat-demo.gif)

- Rename:

	```shell
	$ mv path/to/the/foo.txt path/to/the/bar.txt
	```
	
	In `monat` we can input command below, which is shorter:
	
	```shell
	$ mn path/to/the/foo.txt ,/bar.txt
	```
	
- Move file up one level:

	```shell
	$ mv path/to/another/foo.txt path/to/
	```
	
	In `monat` we can input:
	
	```shell
	$ mn path/to/another/foo.txt ,/../
	```

- Use history:

	Directory visited by `monat`  just now will be stored in the history. Using the `mn` command without arguments will list the history. When in Bash, you can also press `<TAB>` to autocomplete. 

	```shell
	$ mn ,<TAB><TAB>
	1 -- path/to/the
	2 -- path/to/another
	3 -- path/to
	```
	
	Then you can use the number after the comma to represent the path:
	
	```shell
	$ mn ,1/bar.txt ,2
	```

	is equivalent to:
	
	```shell
	$ mv path/to/the/bar.txt path/to/another/
	```
	
- Run another command by leveraging `monat`:

	```shell
	$ mn -c vim ,2/bar.txt
	```
	
	is equivalent to:
	
	```shell
	$ vim path/to/another/bar.txt
	```
	
- Dive into a sub-directory and run a command:

	```shell
	$ cd path/to/the/
	$ make
	$ cd -
	```
	
	Though you can gather them into one line, you can't have comamnd or filename autocompletion though:
	
	```shell
	$ cd path/to/the/ && make; cd -
	```
	
	In `monat` we can input:
	
	```shell
	$ mn -d ,1 -c make
	```
	
	The advantages are:
	
	1. short command in one line, 
	
	2. the `make` command can be autocompleted by pressing `<TAB>`. Also if you want to pass some filenames to the command, `monat` will complete it for you.

Remember you can always press `<TAB>` to complete the path, command and filename in Bash, including the one represented by comma and number, which is very easy to input the whole command. Support for other shells will be added in future.

## Usage

### Syntax

The `monat` syntax is simple. The path starting with a comma (,) is called *Path Epitome*.

If inputing two path with the second one starting with a comma, the comma represents the path prefix of the first one. Just like the first example.

If a comma follows a number `i`, it means the `i`th history record in the history file.

### Arguments

`monat`'s function varies by the number of *Path Epitome* arguments.

1. 0 *Path Epitome*: list the history for the current directory

2. 1 *Path Epitome*: `monat` serves as the `ls` command

3. 2 *Path Epitomes*: `monat` serves as the `mv` command

4. with `-c` argument: run the command passed by `-c` and expand the *Path Epitome*.

### History file

`monat` stores a history for each project in the `.monat/history` file in the root of the project. Before all the operations, first use `mn -l` in the root directory to set the current directory as the root of the project. If no local `monat` history is found, then `monat` will use the history in home directory `~/.monat/history`. 

The number of history records will be limited to 10 and in a First-In-First-Out (FIFO) manner.

## Tradeoffs

`monat` can only be used when there are no filenames starting with a comma. In the real world this kind of filename will be rare so we consider the design workable. 

## Contribution

`monat` needs more features to be a better tool. Future works:

1. support the same arguments of `ls` and `mv`

2. For now the `monat` only supports autocompletion for Bash. We need support for other shells (especially zsh).
