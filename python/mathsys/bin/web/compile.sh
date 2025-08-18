#
#   COMPILE
#

# COMPILE -> GO TO DIR
cd python/mathsys/bin/web

# COMPILE -> HEAD
echo ";;" > all.wat
echo ";;  HEAD" >> all.wat
echo ";;" >> all.wat
echo "" >> all.wat
echo ";; HEAD -> MODULE" >> all.wat
echo "(module" >> all.wat
echo "" >> all.wat
echo ";; HEAD -> IMPORTS" >> all.wat
echo '(import "env" "memory" (memory 0))' >> all.wat
echo '(import "sys" "call1" (func $call1 (param i32 i32)))' >> all.wat
echo '(import "sys" "call60" (func $call60 (param i32)))' >> all.wat
echo "" >> all.wat
echo "" >> all.wat

# COMPILE -> NUMBER
echo ";;" >> all.wat
echo ";;  NUMBER" >> all.wat
echo ";;" >> all.wat
echo "" >> all.wat
cat number/add.wat >> all.wat
echo "" >> all.wat
echo "" >> all.wat
echo "" >> all.wat

# COMPILE -> SYSTEM
echo ";;" >> all.wat
echo ";;  SYSTEM" >> all.wat
echo ";;" >> all.wat
echo "" >> all.wat
cat system/exit.wat >> all.wat
echo "" >> all.wat
echo "" >> all.wat
cat system/write.wat >> all.wat

# COMPILE -> BOTTOM
echo ")" >> all.wat

# COMPILE -> PROCESS
wat2wasm all.wat -r -o all.wasm
wasm-ld -flavor wasm -r all.wasm -o all.o
rm all.wasm

# COMPILE -> RETURN
cd ..
cd ..
cd ..
cd ..