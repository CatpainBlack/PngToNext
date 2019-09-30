## PngToNext
Utilities to convert PNG images to ZX Next formats

###Command Line Arguments

<pre>
Usage:
  target\debug\pngtonext.exe [OPTIONS] PNG [OUTPUT]

Png to ZX Next image converter

Positional arguments:
  png                   Source image file
  output                Converted file

Optional arguments:
  -h,--help             Show this help message and exit
  -c,--crop CROP        Crop image (left,top,width,height)
  -r,--raw              Save the image as raw data
  -2,--sl2              Save the image as a SL2 data (raw layer 2 image data)
  -n,--nxi              the the image as a NXI file
  -a,--asm              Save data as assembly source
  -p,--pal              Save the palette as a .pal file
  -N,--npl              Save the palette as a .npl file
  -e,--pal-prepend      Place palette data at the start of the file
  -b,--pal-append       Place palette data at the start of end file
  -O,--pal-none         Do not include any palette data
  -v,--verbose          Be verbose
</pre>