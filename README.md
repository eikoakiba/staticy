<h1 align="center"><i>staticy</i>, a static blog generator </br>
</h1>

<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License" />
</p>

Hello and greetings to this project, This is a fun and also important project
For myself and others. the main goal of this project is to make a static blog
without writing Verything it manualy, For those who have static websites/blogs
like myself, this tool is able to automate the whole process. Blow is a list of 
all things that you may want to know.

Notice: This project works best if you write your own shell scripts to intract with
The idea is so simple, we have a base system and we can write other small blocks of
code that may be different for others with things like shell scripting languages.
I'm using bash, but you can use whatever you want. In next updates I will add my own
scripts like cehck if the blog generated before or other things.

## Table Of Contents
1. [Introduction](#introduction)
3. [Installation](#installation)
   1. [Building](#building)
   2. [Pre-compiled Binaries](#download-the-pre-binaries)
5. [Contribute](#contribute)
6. [TODO](#todo)

## Introduction
_on the progress_ (soon)

## Installation
There is multiple ways for installing this tool

### Building
Notice: if you want to compile/build the project, the final
Binary may have some bugs, it's because you are installating
The Latest changes of this repository (beta), if you want to
Have a stable version, it is better to use the other ways.

For building You need to have the rust tool chain
You can download rustup and follow the instructions: [rustup](https://rustup.rs)
After installing rust, you can use command blow:
```
Cargo build --release
```
you can run your binary from target/debug/staticy
Notice that if you run it without required items
It will not works, then please read the [Introduction](#introductio) first.

### Download the Pre-Binaries
You can download the compiled versin from github release page (soon)
Or you can download it from the staticy website (soon)

## Contribute
If you like to add or change any features of this project, then please
Make a pull request.

## TODO
1. Add layout feature
2. Better logging system
3. Hot reloading pages
