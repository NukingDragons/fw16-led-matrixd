
This project aims to fully implement the [API of Frameworks LED spacer modules](https://github.com/FrameworkComputer/inputmodule-rs/blob/main/commands.md) as a cross-platform daemon,
with the added ability to render as many types of images and animated images as possible without using
too many resources, all while preventing the spacers from falling asleep. In conjuction to the daemon,
this repository provides a command line utility for interfacing with the matrixes, making scripting of the
matrixes as easy as possible.

![Banner](https://github.com/user-attachments/assets/cb8d762a-c21f-4ddc-86a8-066eb14d6ab1)

# Table of Contents <a name="toc"></a>

1. [**Installation**](#install)
    - [**Arch/Pacman-Based Distros**](#pacman)
	- [**Debian/Ubuntu/Apt-Based Distros**](#apt)
	- [**Windows**](#windows)
2. [**Example Usage**](#usage)
    - [**Determining Ports**](#ports)
	- [**Adjusting Brightness**](#brightness)
	- [**Rendering Images**](#images)
	- [**Clearing the Matrixes**](#clearing)
3. [**Credits & Thanks**](#credits)
	- [**Framework**](#framework)
	- [**TylerDotRar**](#tylerdotrar)
4. [**License**](#license)

# Installation <a name="install"></a>

## Arch/Pacman-Based Distros <a name="pacman"></a>

### Yay

On arch, there are two packages available. One that targets the current release, and the other that pulls directly from git.

![image](https://github.com/user-attachments/assets/0ec04c79-cc33-4e2c-bde5-c0534a228ef5)

Either can be installed like so:

```bash
yay -S fw16-led-matrixd
# or
yay -S fw16-led-matrixd-git
```

### Git & Makepkg

Alternatively, it can be installed with just git and makepkg by cloning directly from the AUR and building it.

```bash
git clone https://aur.archlinux.org/fw16-led-matrixd.git
cd fw16-led-matrixd.git
makepkg -sic
```

[**Return to Table of Contents**](#toc)

## Debian/Ubuntu/Apt-Based Distros <a name="apt"></a>

To install on debian based distros, I have provided a script that will add my GPG key to `/etc/apt/trusted.gpg.d/nukingdragons.gpg`, and will add
the repository to `/etc/apt/sources.list.d/fw16-led-matrixd.list`. Once the repo is added, the package can be installed and updated through normal
apt update/upgrades.

```bash
curl -fsSL https://nukingdragons.github.io/fw16-led-matrixd/deb.sh | bash
```

![image](https://github.com/user-attachments/assets/479dc842-0698-4594-8f41-584355a86db9)

[**Return to Table of Contents**](#toc)

## Windows <a name="windows"></a>

To install on Windows, I have provided a script that will fetch the current release and place it into `C:\Program Files\fw16-led-matrixd`, and
then it will create a system service to run the daemon as well as add the `ledcli` binary to the global path. To upgrade to the latest release,
simply re-run the install script and it will preserve your config and update you to the latest release. This script MUST be run as an administrator,
otherwise the script will refuse to run.

```powershell
irm https://nukingdragons.github.io/fw16-led-matrixd/win.ps1 | iex
```

![image](https://github.com/user-attachments/assets/3757a272-038b-40d9-9f30-8e064b92f062)

[**Return to Table of Contents**](#toc)

# Example Usage <a name="usage"></a>

There are a LOT of commands available that are exposed via the `ledcli` command. This is NOT an exhaustive list.
Below are some of the more common commands and their usage.

## Determining Ports <a name="ports"></a>

To list the ports (read: attached matrixes) currently connected to your system, issue the command `ledcli list`.
The daemon does not need to be running when you issue this command, and the ports it lists are the exact names that you should place into the config file
before starting the daemon.

![image](https://github.com/user-attachments/assets/fd23fcfb-c41d-4d8a-a309-b92a2779302f)

[**Return to Table of Contents**](#toc)

## Rendering Images <a name="images"></a>

One of the biggest motivaters to create this project for me is the ability to specify an image or a gif to render to the spacers. I wanted it to be easily scriptable
so that I could time these changes at the same interval as my desktop background changes. The render command will resize and forcefully grayscale any image within the
`ledcli` command before sending the matrix compatible vector to the daemon. When specifying a matrix, you can pick either the left matrix, the right matrix, both matrixes,
or you can instruct the daemon to treat the matrixes as if they were a single matrix by specifiying "pair".

Each matrix is 9x34, but in pair mode, images will get resized to 18x34. The first 9 columns will go to the left matrix, and the last 9 will get sent to the right matrix.

![image](https://github.com/user-attachments/assets/33ccbfcb-0751-4f59-8549-455ad8c1c5f2)

[**Return to Table of Contents**](#toc)

## Adjusting Brightness <a name="brightness"></a>

To adjust the overall brightness without disturbing the current image/pattern being rendered to the matrixes, you can use the brightness command to both get and set the current
brightness. The value ranges from 0 to 255. You can set each matrix individually, or you can set both at the same time.

![image](https://github.com/user-attachments/assets/b1902018-b5a2-4909-9cb7-38f1a30238e5)

[**Return to Table of Contents**](#toc)

## Clearing the Matrixes <a name="clearing"></a>

Flushing the columns when there are no staged columns will turn off each LED for a given matrix without adjusting the brightness. This can be used to clear the matrixes.

![image](https://github.com/user-attachments/assets/d33c921c-c6c9-4116-ad7f-1866a551c02f)

[**Return to Table of Contents**](#toc)

# Credits & Thanks <a name="credits"></a>

- Thanks to [@FrameworkComputer](https://github.com/FrameworkComputer) for creating such an awesome open-source platform and for making the API public. <a name="framework"></a>

- Thanks to [@TylerDotRar](https://github.com/tylerdotrar) for helping me create the powershell installer for Windows. <a name="tylerdotrar"></a>

[**Return to Table of Contents**](#toc)

---

# License <a name="license"></a>

- [MIT](/LICENSE)
