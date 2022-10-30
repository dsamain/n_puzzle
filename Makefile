
NAME = n-puzzle

all:
	g++ -o generator gen.cpp
	cargo build --release
	@if [ -f "n-puzzle" ]; then rm -f n-puzzle; fi;
	@cp -r target/release/n-puzzle .

clean: 
	rm -rf target

fclean: clean
	rm -f $(NAME)
	rm -f generator 

re: fclean all

generator: 