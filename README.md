# MS Destreamer

* All credits go to [destreamer's developers](https://github.com/snobu/destreamer)
* This build only caters to Windows or Linux distributions. Check out the [original source](https://github.com/snobu/destreamer) if you want to target it for other platforms.
* The only addition to the original is a simple Rust-compiled executable which asks for the input URL. It is compiled from ```main.rs``` located in the ```/src``` folder. ffmpeg has been added and the Node.js dependencies of destreamer have been built so it can be used by anyone out of the box without hassle.

## How to use destreamer?

1. Clone this repository with git
```bash
git clone https://github.com/FongYoong/ms_destreamer.git
```
**OR**: For those unfamiliar with git, click **Clone** and then click **Download ZIP**. Extract the zip to your preferred location.

![title](https://github.com/FongYoong/ms_destreamer/blob/master/images/1.jpg)
***

2. Run the ```ms_destreamer.exe``` file located in the project folder.

![title](https://github.com/FongYoong/ms_destreamer/blob/master/images/2.jpg)
***

3. The program will prompt for the link/URL of the MS-Stream video.
Press ```Enter``` after inserting the link.

![title](https://github.com/FongYoong/ms_destreamer/blob/master/images/3.jpg)
***

4. If it is your first login or your previous auth-token has expired, you will be redirected to Microsoft's login page.

![title](https://github.com/FongYoong/ms_destreamer/blob/master/images/4.jpg)
***

5. If your login was successful, take a Milo break while the video is being downloaded.

![title](https://github.com/FongYoong/ms_destreamer/blob/master/images/5.jpg)
***

6. The downloaded video can be found in the ```/video``` folder.

![title](https://github.com/FongYoong/ms_destreamer/blob/master/images/6.jpg)
***
