ffmpeg -r:v 30 -i "./rendered/video/%d.tiff" -codec:v libx264 -preset veryslow -pix_fmt yuv420p -crf 28 -an "./rendered/results.mp4"
rm -rf "./rendered/video"
