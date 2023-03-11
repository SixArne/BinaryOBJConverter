# How to use

Make sure to install rust
https://www.rust-lang.org/

for debugging symbols install 
`rust-lang.rust-analyzer`

Release exe is found under `target/release`.

## command usage:

cd into the Data folder. If you added the exe to your environment variables use the following command.

### From text obj to bin
>ObjConverter --input bunny.obj --output bunny.bobj

### From bin obj to test
>ObjConverter --input bunny.bobj --output bunny.obj --reverse