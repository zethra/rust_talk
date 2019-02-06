.PHONY: all

SRC=$(wildcard *.rs)

OUT=$(patsubst %.rs,%,$(SRC))

all: $(OUT)

%: %.rs
	rustc -o $@ $<
