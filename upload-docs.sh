#!/bin/bash

cargo doc 

rm -r -f .gh-pages
mkdir .gh-pages
cd .gh-pages
git init

cp -r ../target/doc/* .
cat <<EOF > index.html
<!doctype html>
<title>Documentation</title>
<meta http-equiv="refresh" content="0; ./lib/">
EOF

git add -f --all .
git commit -m "Added docs"
git remote add origin git@github.com:seb-odessa/$(basename $(pwd)).git
git push -f origin master:gh-pages

cd ..
rm -r -f .gh-pages
