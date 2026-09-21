## Snugly

### Total Time Spent :

---

### Journal #1 : Completing the schematics!

#### Total time : 2 hours 5 mins

#### By @Shadow

Starting with the project, the first plan was to make the schematics! For this hackpad, I am designing a PCB with a custom
integrated MCU rather than using a premade module such as the Xiao RP2040, for the MCU, I will be using the beginner friendly
RP2040 MCU, I started by adding the symbol to KiCAD and wiring it up! I used help of the custom devboard guide by Kai Pierra,
and the official datasheet!

| MCU                                                                                                     |
| ------------------------------------------------------------------------------------------------------- |
| <img alt="mcu" src="https://github.com/user-attachments/assets/156aa231-8820-498e-9714-a4bfaf2defcb" /> |

It will be powered by USB C, providing it with 5V which is then stepped down to 3.3V with the use of LDO, other than that it
uses 16Mb quad SPI flash memory making it 4 times as fast as single SPI memory!

| Power and Flash                                                                                           |
| --------------------------------------------------------------------------------------------------------- |
| <img alt="power" src="https://github.com/user-attachments/assets/73deb518-4e30-47de-bbf0-ce59a9a8e555" /> |

The hackpad will have a 3 by 3 matrix of total 9 keys, with individual RGB LED ( WS2812B ) for all the keys! It also consists of
a Rotatory Encoder and 0.91" Inch OLED!

### Journal #2 : Making the Bootup Art

#### Total time : 3h~

#### By @cloudglides

For the bootup screen of the hackpad, I wanted to make some custom art instead of using a basic logo or text. I decided to make an Orpheus themed art for it.

I first made Orpheus holding a booklet, but later changed the idea and made him hold a PCB instead. This fit the project much better since the hackpad is a hardware project and the PCB is an important part of it.

| ophie                                                                                                     |
| --------------------------------------------------------------------------------------------------------- |
| <img alt="power" src="https://github.com/user-attachments/assets/dac15541-5ff2-4e39-a71a-7034ea215981" /> |

I made the art specifically to be used on the hackpads bootup screen.

### Journal #3 : Adding footprints and working on PCB

#### Total time : 2h 15 mins

#### By @Shadow

After being done with the Schematics, I started with adding the footprints, to keep the MCU side compact and easy to route, I am using 0402 footprint for capacitors and resistors, I started routing the PCB, starting with the power side first, I tried to place the decoupling capacitors first, after a few tries I got it!

| PCB                                                                                                                               |
| --------------------------------------------------------------------------------------------------------------------------------- |
| <img  alt="Screenshot 2026-09-20 132101" src="https://github.com/user-attachments/assets/470c22c9-d438-444f-ac52-5f5444417474" /> |

I am trying to keeep all the components on bottom layer of PCB and have a clean routing, and use the top layer for a clean ground plane and power schematics!

| PCB again                                                                                                                                                  |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <img width="1387" height="852" alt="Screenshot 2026-09-20 141342" src="https://github.com/user-attachments/assets/9946e197-53ea-47de-8583-901767672858" /> |

### Journal #4 : Writing firmware and working on canva

#### Total time : 5h~

#### By @cloudglides

im done firmware pretty much... its 5AM and im not right on mind? i believe i used a rust lib to write the firmware from scratch (not technically)- but okay i also tried writing some unit tests in `lib.rs` but apparently you cant really write decent unit tests in hw so i gave up, (should have known earlier)... about canva i was working on a zine to use later in a readme but im not really complete and really tired for my mind to work decently... im so glad my exam is tommorow otherwise i would have fumbled today really bad- leaving that apart my thought process in canva is really pretty much FAFO... i try alot of new things remove them and iterate upon them again and thats how i make my good softwares... about rust it wont work unless you have rustup installed and are a nerd since you also have to resolve some path... pretty much why i am nit a big fan of this but it is what it is

### Journal #5 : Writing firmware

#### Total time : 3h~

#### By @cloudglides

worked on firmware once again, pretty much again worked on making it more efficient and running some animations on the oled.. pretty fun though im not aware about how good it will look as i dont have access to the hardware
apart from that i did very little art, needed a few iterations on the original work
