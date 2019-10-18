### PngToNext
Utility to convert PNG images to various ZX Next formats

####Command Line Arguments

<pre>
Usage:
  pngtonext [OPTIONS] PNG [OUTPUT]

Png to ZX Next image conversion

Positional arguments:
  png                   Source image file
  output                Converted file

Optional arguments:
  -h,--help             Show this help message and exit
  -c,--crop CROP        Crop image (left,top,width,height)
  -r,--raw              Save the image as raw data
  -2,--sl2              Save the image as a SL2 data (raw layer 2 image data)
  -n,--nxi              Save the the image as a NXI file
  -a,--asm              Save data as assembly source
  -p,--pal              Save the palette as a .pal file
  -N,--npl              Save the palette as a .npl file
  -s,--slr              Save the the image as a slr file
  -P,--prepend-palette  Place palette data at the start of the file
  -A,--append-palette   Place palette data at the start of end file
  -O,--no-palette       Do not include any palette data
  -v,--verbose          Be verbose
</pre>
