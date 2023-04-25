# run in `reversi-bot`

wasm-pack build --target web
rm -r www/src/pkg
cp -r pkg www/src
rm www/src/pkg/.gitignore