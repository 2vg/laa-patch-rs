laa-patch-rs
===

Easily apply LAA patches to EXE files.  
If the LAA flag for Windows 32-bit applications is not enabled, Windows 32-bit applications will not be able to handle more than 2GB of memory on 64-bit Windows.  
This can cause bugs and crashes in older 32-bit applications.  
To fix this, patch the binaries directly and enable the LAA flag.  
This flag is determined at run time, so I think there is no problem with the compiled binary.  

## usage

run `laa-patch-rs.exe`, then choose the EXE file that you want to LAA patch.
You can optionally choose to output the patch application as a othet EXE file or overwrite itself.

## Credit

~~**EXEのパッチ気持ちよすぎだろ！**~~
