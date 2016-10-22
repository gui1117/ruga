#!/bin/sh

cd $(dirname $0)

mkdir target 2> /dev/null
cd target

rm -r publication 2> /dev/null
mkdir publication
cd publication

# x86_64-unknown-linux-gnu

echo 'do x86_64-unknown-linux-gnu'

mkdir ruga
cd ruga

(cd ../../../; cargo build --release)
cp ../../release/ruga .
cp -r ../../../README.md ../../../config.toml ../../../assets ../../../levels .

echo '''#!/bin/sh

cd $(dirname $0)
./ruga''' > launch.sh
chmod a+x launch.sh

cd ..
tar -czvf ruga_linux64.tar.gz ruga
rm -r ruga

echo 'done x86_64-unknown-linux-gnu'

# x86_64-pc-windows-gnu

echo 'do x86_64-pc-windows-gnu'

mkdir ruga
cd ruga

(cd ../../../; cargo build --release --target x86_64-pc-windows-gnu)
cp ../../x86_64-pc-windows-gnu/release/ruga.exe .
cp -r ../../../README.md ../../../config.toml ../../../assets ../../../levels .

cd ..
zip -r ruga_win64.zip ruga
rm -r ruga

echo 'done x86_64-pc-windows-gnu'
