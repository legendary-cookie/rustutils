#!/bin/bash
mkdir -p build/; cd build/
files=""
outdir="out/"
mkdir -pv $outdir

echo " =================== "
echo "  Finding Packages   "
echo " =================== "
echo

for f in $(find .. -name PKGBUILD.\*)
do
	echo "=> Found $(basename $f)"
	cp $f .
	files="$files $(basename $f)"
done	

echo
echo " =================== "
echo "  Building Packages  "
echo " =================== "
echo

for f in $files
do
	echo "=> Building $(echo "$f" | sed 's/PKGBUILD\.//g')"
	F=$(realpath $f)
	makepkg --sign --skipinteg --clean -sp $F >/dev/null
	mv *.tar.* $outdir
	rm -rf "$(echo "$f" | sed 's/PKGBUILD\.//g')"
done
