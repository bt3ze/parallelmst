all: mst images

mst: mst.rs
	rustc mst.rs

images: gradient sample corners 

gradient: ImageReader.class ImageReader.java
	java ImageReader step_gradient.jpg

sample: ImageReader.class ImageReader.java
	java ImageReader sample.jpg

corners: ImageReader.class ImageReader.java
	java ImageReader small-corners.jpg
