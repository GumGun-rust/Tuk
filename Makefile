b:
	@printf "\n\n\n\n\n\n\n\n\n"
	@printf "=========================================="
	@printf "\n\n\n\n\n\n\n\n\n"
	cargo build

t:
	cargo test -- --show-output


holder:
	alias T="./tuk 2> out"
	alias TE="./tuk test/testSrc.txt 2> out"
