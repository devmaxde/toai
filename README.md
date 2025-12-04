# toai

toai is a tiny Rust CLI that dumps a folder into an AI-friendly text format.  
Each file is printed with its relative path header, followed by its contents.  
**By default, the output is copied directly to your clipboard** – ready to paste into any AI chat.

Example of the output format:

# src/main.tsx

-- file content here --

# package.json

-- file content here --

By default, toai skips heavy folders (node_modules, target, .git, …) and common binary/cache files via glob patterns.

---

## Install

From source:

1. git clone https://github.com/devmaxde/toai.git
2. cd toai
3. cargo install --path .

or

1. cargo install toai

---

## Usage

toai [OPTIONS]

Common examples:

1. Dump current directory to clipboard (default)  
   toai --path .

2. Dump a specific folder to clipboard  
   toai --path ./my-project

3. Write output to a file  
   toai --path . --output output.txt

4. Print to stdout (for piping)  
   toai --path . --stdout

5. Add custom ignores (repeatable, supports globs)  
   toai --path . --ignore somefolder --ignore "\*.png" --ignore "\*.log"

6. Disable default ignores and specify your own  
   toai --path . --no-ignore-default --ignore build --ignore cache --ignore "\*.o"

7. Ensure the output file isn't included (handled automatically if it's inside the scanned tree)  
   toai --path . --output ./dump/project.txt

---

## Options

--path <PATH>  
Root directory to scan (default: .)

--output <FILE>  
Write output to a file instead of clipboard

--stdout  
Print output to stdout instead of clipboard (useful for piping)

--ignore <PATTERN>  
Ignore entries matching a pattern. Supports globs like \*.png, \*\*/dist, cmake-build-\*  
Can be provided multiple times

--no-ignore-default  
Disable the default ignore set

---

## Default ignore set

Applied unless --no-ignore-default is used.

Directories  
node*modules  
target  
dist  
build  
.next  
.turbo  
.git  
.idea  
.vscode  
**pycache**  
.pytest_cache  
.mypy_cache  
.ruff_cache  
CMakeFiles  
cmake-build-*  
Pods  
buck-out  
bazel-\_

Files  
Cargo.lock  
LICENSE  
.DS_Store

Binary and cache extensions  
Python: _.pyc, _.pyo, _.pyd  
C/C++/Rust: _.o, _.obj, _.so, _.dll, _.dylib, _.exe, _.out, _.a, _.lib  
Logs/Temp: _.log, _.tmp, _.swp  
Images: _.png, _.jpg, _.jpeg, _.gif, _.bmp, _.tiff, _.ico, _.svg, _.webp, _.heic, _.heif  
3D assets: _.vrm, _.fbx, _.glb, _.gltf, _.blend, _.obj, _.stl  
Archives: _.zip, _.tar, _.gz, _.bz2, _.xz, _.7z, _.rar

---
