# kernel/Makefile
CC      := gcc
AR      := ar
CFLAGS  := -std=c11 -Wall -Wextra -Werror -O2 -Iinclude

OBJS    := src/shbt_ecc.o src/shbt_swarm_kernel.o
TARGET  := libshbt_swarm_kernel.a

.PHONY: all clean test

all: $(TARGET)

%.o: %.c include/shbt_swarm_mmio.h include/shbt_ecc.h
	$(CC) $(CFLAGS) -c $< -o $@

$(TARGET): $(OBJS)
	$(AR) rcs $@ $(OBJS)

test: all
	$(CC) $(CFLAGS) -I../include ../tests/reference_test.c -L. -lshbt_swarm_kernel -o run_test
	./run_test
	rm -f run_test run_test.exe

clean:
	rm -f $(OBJS) $(TARGET) run_test run_test.exe
