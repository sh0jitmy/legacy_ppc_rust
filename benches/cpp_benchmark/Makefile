CXX = powerpc-linux-gnu-g++
CXXFLAGS = -O3 -mcpu=8548 -mabi=spe -mspe -Wall -std=c++17
LDFLAGS = -pthread -lm

TARGET = cpp_benchmark

all: $(TARGET)

$(TARGET): main.cpp
	$(CXX) $(CXXFLAGS) -o $@ $< $(LDFLAGS)

clean:
	rm -f $(TARGET)

.PHONY: all clean
