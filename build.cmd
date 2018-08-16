cargo web build --target=wasm32-unknown-unknown --release
copy target\wasm32-unknown-unknown\release\art_pattern.js html\art_pattern.js
copy target\wasm32-unknown-unknown\release\art_pattern.wasm html\art_pattern.wasm
rem "请打开html\index.html查看运行结果"