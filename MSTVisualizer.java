import java.awt.image.BufferedImage;
import java.awt.image.WritableRaster;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.util.Collections;
import java.util.Scanner;
import java.util.Vector;

import javax.imageio.ImageIO;



public class MSTVisualizer {
	
	//UPDATE HERE: Yields 8 result images
	private final static double startPercentile = 0.75;
	private final static double step = 0.03;
	

	@SuppressWarnings("unchecked")
	public static void main(String[] args) {

		//Tools and properties
		File input = null;
		BufferedImage image = null;
		WritableRaster wr = null;

		//Data structures
		float [] pixelArray = null;
		Vector<Edge> edges = new Vector<Edge>();

		//Variables
		int height = 0; 
		int width = 0;


		try {

			//Read and parse input
			System.out.println("------------------------------------INPUT------------------------------------");
			input = new File(args[0]);
			//input = new File("src/sample.out.txt");
			System.out.println("File opened: "+ input.getPath());
			Scanner reader = new Scanner(new FileInputStream(input));


			int malformedLinesSkipped = 0;
			int edgesAdded = 0;
			while (reader.hasNextLine()) {      
				String line = reader.nextLine();
				//Parse out the image dimensions
				if(line.contains("H") && line.contains("W")){
					int hpos = line.indexOf("H");
					int wpos = line.indexOf("W");
					height = Integer.parseInt(line.substring(hpos+1,wpos).trim());
					width = Integer.parseInt(line.substring(wpos+1).trim());
				}

				//Parse out all well-formed edge data into edges[]
				if(line.startsWith("(")){ //(0,0),(1,0):0
					String coords = line.split(":")[0];
					String [] components = coords.split(",");
					try{
						int startx = Integer.parseInt(components[0].substring(1));
						int starty = Integer.parseInt(components[1].substring(0,components[1].length()-1));
						int endx = Integer.parseInt(components[2].substring(1));
						int endy = Integer.parseInt(components[3].substring(0,components[3].length()-1));
						float cost = Float.parseFloat(line.split(":")[1]);

						edges.add(new Edge(startx,starty,endx,endy,cost));
						edgesAdded++;
					}
					catch(Exception e){
						malformedLinesSkipped++;
					}
				}
			}

			System.out.println("Edges added: "+edgesAdded);
			System.out.println("Malformed lines skipped: "+malformedLinesSkipped);

			//Sort edge vector by cost
			Collections.sort(edges);


		} catch (IOException e) {
			e.printStackTrace();
			System.out.println("Unable to parse edge data from file.");
		}


		//Initialize output image buffer
		int imageType = 5;
		image = new BufferedImage(width, height, imageType);
		wr = (WritableRaster) image.getData();
		System.out.println("Image dimensions: H"+height+"W"+width);
		System.out.println("------------------------------------OUTPUT------------------------------------");


		//Image Processing
		if(wr != null && image != null && !edges.isEmpty()){
			int iter = 0;
			for(double percentile = startPercentile; percentile < 1; percentile += step){
				
				//Set all pixels initially to white
				for(int h = 0; h < height; h++){
					for(int w = 0; w < width; w++){
						wr.setPixel(w, h, Color(255,255,255));
					}
				}
				
				
				//Percentile calculation of heavy-cost cutoff
				int index = (int) (edges.size()*percentile);
				Edge cutoff = edges.get(index);
				float heavyCutoff = cutoff.getCost();
				//float heavyCutoff = (float) 50; //230.0;
				System.out.println("Heavy cost cutoff applied: "+heavyCutoff);
				

				//Search for heavy edges
				int heavyEdges = 0;
				for(int i = 0; i < edges.size(); i++){
					if(edges.get(i).getCost() > heavyCutoff){                                        
						int startX = edges.get(i).getStartX();
						int startY = edges.get(i).getStartY();
						int endX = edges.get(i).getEndX();
						int endY = edges.get(i).getEndY();

						wr.setPixel(startX, startY, Color(0,0,0));
						wr.setPixel(endX, endY, Color(255,0,0));

						heavyEdges++;
					}
				}
				System.out.println("Heavy edges found: "+heavyEdges);		


				//Depth-first search to recolor pixels
				/*for(int i = 0; i < edges.size(); i++){
					//System.out.println(edges.get(i).toString());
				}*/




				//Write output to files
				pixelArray = wr.getPixels(0, 0, width, height, pixelArray);
				wr.setPixels(0, 0, width, height, pixelArray); 
				image.setData(wr);
				try {
					String filename = "outputImages/out"+iter+".jpg";
					ImageIO.write(image, "jpg", new File(filename));
					System.out.println("Results successfully written to "+filename+".");
				} catch (IOException e) {
					System.out.println("Unable to write results for cutoff of "+heavyCutoff+"to an image file.");
				}
				System.out.println("---------");
				iter++;
			}
		}
		else{
			System.out.println("Error initializing data structures.");
		}		

		System.out.println("Done");
	}

	private static float[] Color(int r, int g, int b){
		return new float[]{(float)r,(float)g,(float)b};
	}

}