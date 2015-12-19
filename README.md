# diosix

[![Build Status](https://travis-ci.org/diodesign/diosix.svg?branch=master)](https://travis-ci.org/diodesign/diosix)

This is a lightweight and secure 64-bit multiprocessor microkernel operating system written in Rust for x86, ARM and MIPS systems.
It's a work in progress: I'm starting from scratch after [previously writing](https://github.com/diodesign/diosix-legacy)
a working microkernel for 32-bit SMP x86 computers in C and assembly.

I learned a lot from that first iteration, and this is the second iteration of diosix, codenamed Menchi. Crucially,
it will be written [in Rust](https://www.rust-lang.org/), a C/C++-like programming language that has a fierce emphasis
on guaranteed memory safety, threads without data races, and other security features.

Check out [the wiki for documentation](https://github.com/diodesign/diosix/wiki) on how it all works internally.

### Building

These are the tools I've got installed on a Debian "Jessie" GNU/Linux server for building diosix; other versions of the software are probably fine, too:

* `cargo 0.7.0-nightly`
* `GNU ld 2.25`
* `GNU make 4.0`
* `grub-mkrescue 2.02`
* `nasm 2.11.05`
* `multirust 0.7.0` (for installing Rust)
* `qemu 2.1.2` (for testing)
* `rustc 1.6.0-nightly`
* `vim` ;-)

To build the software, open a terminal, check out the source code in the usual way, and change into its directory:

```
git clone https://github.com/diodesign/diosix.git
cd diosix
```

Then pick a hardware platform to build a kernel for. Let's start with 64-bit x86, aka a standard PC machine. Change into its directory:

```
cd platform/x86
```

Next, make sure you're using a suitable Rust toolchain: you'll need a nightly build as we need features not present in the stable edition:

```
multirust override nightly
```

Now build a bootable ISO image suitable for burning to a CD/DVD or throwing at an emulator or hypervisor to test:

```
make iso
```

The ISO should be saved in the platform's release directory - in this case: `diosix/release/x86/boot.iso`.
To fire up the ISO image in QEMU:

```
make run
```

If I haven't broken the kernel, the emulator will start up in your ncurses-friendly terminal, and boot the tiny operating system.
QEMU will be automatically killed after a timeout period, typically 10 seconds. This is because the kernel cannot, right now, power off the 
system; an ACPI driver to handle power management will eventually be written, and until then we have to kill QEMU manually. If you want to see
the kernel's detailed debugging output via the serial port, boot QEMU headless, and the serial port will be logged to the terminal.

```
make run-headless
```

Finally, `make clean` removes the debris left behind by the build process.

### Screenshot

Here's a very early build of diosix booting on x86.

![Screenshot of QEMU running diosix](https://raw.githubusercontent.com/diodesign/diosix/screenshots/docs/screenshots/diosix-early-1.png)

And here's the kernel's debugging output after its physical and virtual memory allocator was implemented – this output can be seen by running `make run-headless`.

![Screenshot of QEMU running diosix](https://raw.githubusercontent.com/diodesign/diosix/screenshots/docs/screenshots/diosix-mem-1.png)

### Contact

Feel free to [email me](mailto:diodesign@gmail.com), Chris Williams, if you have any questions, want to get involved, have source to contribute, or found a security flaw.
You can also find me, diodesign, on [Freenode IRC](https://freenode.net/irc_servers.shtml) in the #osdev channel, or [on Twitter](https://twitter.com/diodesign).

### Copyright, license, and thanks

&copy; Chris Williams and contributors, 2015. See LICENSE for source code and binary distribution and use.

With thanks to Philipp Oppermann for his guide to [linking Rust code](http://os.phil-opp.com/setup-rust.html) to low-level bare-metal assembly,
and to the OSdev community for its [notes and documentation](http://wiki.osdev.org/Main_Page).

