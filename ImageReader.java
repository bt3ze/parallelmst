//adapted from http://stackoverflow.com/questions/6524196/java-get-pixel-array-from-image

import java.awt.image.BufferedImage;
import java.awt.image.DataBufferByte;
import java.io.IOException;
import javax.imageio.ImageIO;
import java.io.File;
import java.io.BufferedWriter;
import java.io.FileWriter;

public class ImageReader {

   private static final String[] IMAGE_EXTS = { "jpg", "jpeg" };

   public static void main(String[] args) throws IOException {
   	  System.out.println("Filename: " + args[0]);

   	  //Verify the resource type
   	  File file = new File(args[0]);
   	  String filename = file.getName();
   	  String title = filename.substring(0, filename.lastIndexOf('.'));

   	  boolean valid = false;
      for(int i = 0; i < IMAGE_EXTS.length; i++){
           if(filename.endsWith(IMAGE_EXTS[i])){
        		valid = true;
           }
       }

       if(!valid) System.exit(0);


      //Process the resource
      BufferedImage hugeImage = ImageIO.read(ImageReader.class.getResource(filename));

      long startTime = System.nanoTime();
      int[][] result = readRGB(hugeImage);
      long endTime = System.nanoTime();
      System.out.println(String.format("Processing time: %s", timeToString(endTime - startTime)));
      
      //System.out.println(imageToString(result));
      imageToFile(result, title+".txt");
      System.out.println("Image data recorded successfully");
   }

   private static int[][] readRGB(BufferedImage image) {

      final byte[] pixels = ((DataBufferByte) image.getRaster().getDataBuffer()).getData();
      final int width = image.getWidth();
      final int height = image.getHeight();
      final boolean hasAlphaChannel = image.getAlphaRaster() != null;

      //use bitshifting to efficiently encode RGB(A) values for pixels
      int[][] result = new int[height][width];
      if (hasAlphaChannel) {
         final int pixelLength = 4;
         for (int pixel = 0, row = 0, col = 0; pixel < pixels.length; pixel += pixelLength) {
            int argb = 0;
            argb += (((int) pixels[pixel] & 0xff) << 24); // alpha
            argb += ((int) pixels[pixel + 1] & 0xff); // blue
            argb += (((int) pixels[pixel + 2] & 0xff) << 8); // green
            argb += (((int) pixels[pixel + 3] & 0xff) << 16); // red
            result[row][col] = argb;
            col++;
            if (col == width) {
               col = 0;
               row++;
            }
         }
      } else {
         final int pixelLength = 3;
         for (int pixel = 0, row = 0, col = 0; pixel < pixels.length; pixel += pixelLength) {
            int argb = 0;
            argb += -16777216; // 255 alpha
            argb += ((int) pixels[pixel] & 0xff); // blue
            argb += (((int) pixels[pixel + 1] & 0xff) << 8); // green
            argb += (((int) pixels[pixel + 2] & 0xff) << 16); // red
            result[row][col] = argb;
            col++;
            if (col == width) {
               col = 0;
               row++;
            }
         }
      }

      return result;
   }

   private static String timeToString(long nanoSecs) {
      int minutes    = (int) (nanoSecs / 60000000000.0);
      int seconds    = (int) (nanoSecs / 1000000000.0)  - (minutes * 60);
      int millisecs  = (int) ( ((nanoSecs / 1000000000.0) - (seconds + minutes * 60)) * 1000);


      if (minutes == 0 && seconds == 0)
         return millisecs + "ms";
      else if (minutes == 0 && millisecs == 0)
         return seconds + "s";
      else if (seconds == 0 && millisecs == 0)
         return minutes + "min";
      else if (minutes == 0)
         return seconds + "s " + millisecs + "ms";
      else if (seconds == 0)
         return minutes + "min " + millisecs + "ms";
      else if (millisecs == 0)
         return minutes + "min " + seconds + "s";

      return minutes + "min " + seconds + "s " + millisecs + "ms";
   }

   private static String imageToString(int[][] array){
   	  int h = array.length;
   	  if(h == 0) return "";
   	  int w = array[0].length;
   	  if(w == 0) return "";
   	  System.out.println("Pixel dimensions: " + h + "x" + w);

   	  String retVal = "";
   	  for(int row = h-4; row < h; row++){
   	  	for(int col = w-4; col < w; col++){
   	  		int rgb = array[row][col];
   	  		int red = (rgb >> 16) & 0x000000FF;
			int green = (rgb >> 8 ) & 0x000000FF;
			int blue = (rgb) & 0x000000FF;
   	  		retVal = retVal + "R" + red + "G" + green + "B" + blue + "\t";
   	  	}
   	  	retVal += "\n";
   	  }
   	  return retVal;
   }

   private static void imageToFile(int[][] array, String filename){
   	  int h = array.length;
   	  if(h == 0) System.exit(0);
   	  int w = array[0].length;
   	  if(w == 0) System.exit(0);

   	  try{
   	  	BufferedWriter outputWriter = new BufferedWriter(new FileWriter(filename));

   	  	//write array dimensions
   	  	outputWriter.write("H" + h + "W" + w);
   	  	outputWriter.newLine();

   	  	//write pixel data on each line
   	  	for(int row = 0; row < h; row++){
   	  		for(int col = 0; col < w; col++){
   	  			int rgb = array[row][col];
   	  			int red = (rgb >> 16) & 0x000000FF;
				int green = (rgb >> 8 ) & 0x000000FF;
				int blue = (rgb) & 0x000000FF;

				String pixelData = "R" + red + "G" + green + "B" + blue;
				outputWriter.write(pixelData);
				outputWriter.newLine();
   	  		} 
   	  	}
   	  	outputWriter.flush();  
  	  	outputWriter.close(); 
  	  }catch(IOException e){
  	  	System.out.println("Error writing to file");
  	  	System.exit(0);
  	  }
   }


}