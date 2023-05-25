#!/bin/bash

# ------- copy IDLs into apps or run this script from directory 'core' with command ./scripts/cp_idl.sh
# bank
#cp ./target/idl/gem_bank.json ./app/gem-bank/public/
# farm
#cp ./target/idl/gem_bank.json ./app/gem-farm/public/
# cp ./target/idl/gem_farm.json ./app/gem-farm/public/

# ------- copy types into SDK
cp -r ./target/types ./src/

echo IDLs and Types copied!
