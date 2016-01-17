all: hello.hex

hello.o: hello.rs
	rustc --target avr-atmel-none -C target-cpu=atmega328p --emit=obj $< -o $@

# Use GCC to link main function into a complete binary
hello.elf: hello.o
	avr-gcc -mmcu=atmega328p $< -o $@

# Convert binary to an Intel HEX file for upload
hello.hex: hello.elf
	avr-objcopy -O ihex -R .eeprom $< $@

# Download the HEX to the board
.PHONY: program
program: hello.hex
	avrdude -p atmega328p -c arduino -P /dev/cu.usbmodem1411 -U flash:w:$<:i
