#!/bin/bash

rm -fr udata
mkdir udata
pushd udata

wget https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry

wget ftp://unicode.org/Public/15.1.0/ucd/Blocks.txt
wget ftp://unicode.org/Public/15.1.0/ucd/NamesList.txt
wget ftp://unicode.org/Public/15.1.0/ucd/UnicodeData.txt

popd
