
NAME = n-puzzle

all:
	cargo build --release
	mv target/release/$(NAME) $(NAME)

clean: 
	rm -rf target

fclean: clean
	rm -f $(NAME)

re: fclean all