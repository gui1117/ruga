#!/bin/sh

mkdir target 2> /dev/null
cd target

rm -r publication 2> /dev/null
mkdir publication
cd publication

# x86_64-unknown-linux-gnu

echo 'do x86_64-unknown-linux-gnu'

mkdir 'x86_64-unknown-linux-gnu'
cd 'x86_64-unknown-linux-gnu'

(cd ../../../; cargo build --release)
cp ../../release/ruga .
cp -r ../../../README.md ../../../config.toml ../../../assets ../../../levels .

echo '''#!/bin/sh

cd $(dirname $0)
./ruga''' > launch.sh
chmod a+x launch.sh

echo 'done x86_64-unknown-linux-gnu'

