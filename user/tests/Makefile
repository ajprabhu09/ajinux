ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

TOP_DIR = $(ROOT_DIR)
INC_DIR = $(ROOT_DIR)/../lib/inc

CC=/opt/homebrew/Cellar/x86_64-unknown-linux-gnu/7.2.0/bin/x86_64-unknown-linux-gnu-gcc
LD=/opt/homebrew/Cellar/x86_64-unknown-linux-gnu/7.2.0/bin/x86_64-unknown-linux-gnu-ld
FLAGS = -I$(INC_DIR) 
LDFLAGS = 

SRC := $(TOP_DIR)/progs

TESTS := simple
OBJ := $(patsubst %,$(SRC)/%.o,$(TESTS))
PROG := $(patsubst %,$(SRC)/%.out,$(TESTS))


# TODO: add -c
$(SRC)/%.o: $(SRC)/%.c
	$(CC) $(FLAGS) -c $< -o $@ 

$(SRC)/%.out: $(SRC)/%.o $(SRC)/../crt0.o
	@printf "\n\nBuilding TEST $@ \\n\\n"
	$(LD) -o $@ $(SRC)/../crt0.o  $< 




# $(TOP_DIR)/%: $(TOP_DIR)/%/%.o`

all: $(PROG) $(SRC)/../crt0.o
clean:
	rm -rf $(SRC)/*.o
	rm -rf $(SRC)/*.out
	rm -rf $(TOP_DIR)/crt0.o