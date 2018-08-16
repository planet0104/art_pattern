cargo web build --target=wasm32-unknown-unknown --release
copy target\wasm32-unknown-unknown\release\art_pattern.js art_pattern.js
copy target\wasm32-unknown-unknown\release\art_pattern.wasm art_pattern.wasm
rem "请打开index.html查看运行结果"