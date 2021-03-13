#!/bin/bash
cd assembler;
cargo clean;
cargo test;
cargo build --release;
cd ../;
for i in `ls */*.asm`; do
	../../tools/Assembler.sh $i;
	./assembler/target/release/assembler $i "test.hack";
	if diff "${i%.asm}.hack" "test.hack"
	then
		echo "Successful cmp $i";
	else
	    echo "Failure at $i";
			exit 
	fi
done
rm "test.hack";
