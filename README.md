# newmakefile-rs
Tool to quickly create makefiles based on user-defined templates

## Build
Clone the repo and build with cargo:
```
git clone https://github.com/fracerro/newmakefile-rs
cd newmakefile-rs
cargo build --release
```

Then move the executable into /usr/local/bin:
```
mv target/release/newmakefile
```

## Configuration
Templates must be placed in $HOME/.config/newmakefile.
A template needs two files:
* *template-name*.json -> where to put keywords and the text file name
* *text-file* -> where to put the actual text with keywords that will be replaced

An example can be found in the example-template folder.
