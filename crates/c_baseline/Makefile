CC = powerpc-linux-gnu-gcc
CFLAGS = -O2 -mcpu=8548 -mabi=spe -mspe -Wall
LDFLAGS = -pthread -lm

TARGET = c_baseline

all: $(TARGET)

$(TARGET): main.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

clean:
	rm -f $(TARGET)

.PHONY: all clean
