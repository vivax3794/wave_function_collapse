rm -rfv builds/linux
rm -fv builds/linux.zip
mkdir builds/linux

echo "BUILDING LINUX"
cross build --target x86_64-unknown-linux-gnu --release
cp -v  target/x86_64-unknown-linux-gnu/release/wfc builds/linux
cp -vr assets builds/linux

echo "PACKACING LINUX"
cd builds/linux
zip -r ../linux.zip *
cd ../..