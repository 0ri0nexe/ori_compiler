rm .\test.exe
cd F:\programmation\c\C++\projets\ori_compiler\tests\
nasm.exe -f win64 test.asm -o test.obj
golink.exe test.obj /entry main /console kernel32.dll
.\test.exe
echo $LastExitCode