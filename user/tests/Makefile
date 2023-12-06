TOP_DIR = ./user/tests
INC_DIR = ./user/lib

PROGS := simple 



CC=clang
LD=ld
FLAGS = --target=x86_64-unknown-none -I$(INC_DIR) 
LDFLAGS = 

SRC := $(TOP_DIR)/progs

TESTS := simple
PROG := $(patsubst %,$(SRC)/%.o,$(TESTS))


# PROG = $(SRC)/simple.o
# $(TOP_DIR)/crt0.o: $(TOP_DIR)/crt0.c
# 	$(CC) $(FLAGS) -c $<  -o $@ 
# TODO: add -c
$(SRC)/%.o: $(SRC)/%.c
	$(CC) $(FLAGS) -c $< -o $@ 

# $(TOP_DIR)/%: $(TOP_DIR)/%/%.o`

all: $(PROG)	
clean:
	rm -rf $(SRC)/*.o
	rm -rf $(SRC)/*.out
	rm -rf $(TOP_DIR)/crt0.o