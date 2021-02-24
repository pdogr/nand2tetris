// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.
(start)
	@i
	M=0

(loop)
	@i
	D=M

	@8192
	D=D-A
	@start
	D;JEQ

	@KBD
	D=M
	@color_black
	D;JGT
	@color_white
	D;JEQ

(loop_end)
	@i
	D=M+1
	M=D
	@loop
	0;JMP

	
(color_black)
	@SCREEN
	D=A
	@i
	A=D+M
	M=-1
	@loop_end
	0;JMP

(color_white)
	@SCREEN
	D=A
	@i
	A=D+M
	M=0
	@loop_end
	0;JMP



