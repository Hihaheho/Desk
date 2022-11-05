---
status: accepted
date: 2022-11-06
deciders: Ryo Hirayama
---
# We don't need files and directories

## Context and Problem Statement

Desk Kernel needs a kind of file system to manage them.
The system is used for permission control.

## Decision Drivers

* Is it minimalistic?
* Is it easy to use?
* Is it intuitive?

## Considered Options

* We have files and directories
* We have files
* We have no files and directories

## Decision Outcome

Chosen option: "We don't have files and directories", because it's minimalistic and easy to use.
To have a kind of file system, each AST Node can have a hierarchy.
We call it "the hierarchy system".

## Pros and Cons of the Options

### We have files and directories

* Good, because it likes a computer's file system.
* Bad, because it's hard for users that are not familiar with computers.

### We have files

A file can be like a directory.

* Good, because it's sophisticated.

### We don't have files and directories

Each AST Node is like a file or a directory.

* Good, because it's minimalistic and next state-of-art.
* Good, because it has intuitive permission control.
* Bad, because it's not intuitive for now.

## More Information

- [this article](https://fkohlgrueber.github.io/blog/tree-structure-of-file-systems/) considers current file systems' problems, and says "I would seriously consider using this simple and general node-based structure and drop the separation between files and folders.".
- Unity's hierarchy system is like our system. There is no file and directory, and each GameObject is like a file or a directory.
