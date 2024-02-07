# Braineater
#### Notice! "Brainfuck" is the academically accepted name of the language, and I mean no offense.

## What is Brainfuck?
Brainfuck is a stack-based low-level esoteric programming language invented in the 1990s by Urban Müller. The goal of the project was to create a minimal language with the smallest compiler possible while still being Turing complete. The result is a language that is incredibly hard to use but very entertaining to write.

## About
Braineater is a Brainfuck compiler written in Rust, developed as an introduction to compiler design and generating assembly code. Currently, it only supports macOS with M1 architecture.
This project was really fun; I used Rust to write this compiler

## Why Braineater?
I think the name sounds funny, like the compiler is a zombie. Writing Brainfuck code certainly made me feel like a zombie.

## AArch64 Mac Assembly?
AArch64 or ARM64 (interchangeably) is a  64-bit RISC (reduced instruction set computer) CPU architecture that is used by the Mac M1 and M2 CPUs. Every computer has a CPU that uses machine-specific binary instructions to direct the CPU and therefore the actions of the computer. To create these binary instructions, we use assembly code, which is a set of mnemonics that are short abbreviations or words that are nearly 1-to-1 representations of binary instructions for the CPU. AArch64 (like other architectures) has a language that is specifically analogous to its binary instructions, which is ARM64 assembly. It is important to distinguish Apple ARM64 assembly from Linux ARM64 assembly because of the way in which Apple uses their registers compared to Linux. There are other differences, but this one was by far the most challenging as the documentation is limited, in my opinion.
