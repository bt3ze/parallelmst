
public class Edge implements Comparable{
	private int startX;
	private int startY;
	private int endX;
	private int endY;
	private float cost;

	public Edge(int x1, int y1, int x2, int y2, float c){
		setStartX(x1);
		setStartY(y1);
		setEndX(x2);
		setEndY(y2);
		setCost(c);
	}

	private void setCost(float c) {
		this.cost = c;		
	}

	private void setEndY(int y2) {
		this.endY = y2;		
	}

	private void setEndX(int x2) {
		this.endX = x2;
	}

	private void setStartY(int y1) {
		this.startY = y1;
	}

	private void setStartX(int x1) {
		this.startX = x1;
	}
	
	public float getCost() {
		return this.cost;		
	}

	public int getEndY() {
		return this.endY;		
	}

	public int getEndX() {
		return this.endX;
	}

	public int getStartY() {
		return this.startY;
	}

	public int getStartX() {
		return this.startX;
	}
	
	@Override
	public String toString(){
		return "Start: ("+this.startX+","+this.startY+"), End: ("+this.endX+","+this.endY+"), Cost: "+this.cost;
	}

	@Override
	public int compareTo(Object arg0) { //returns negative for this is < other, 0 for =, positive for >
		float diff = this.cost - ((Edge)arg0).getCost();
		if (diff < 0) return -1;
		else if (diff > 0) return 1;
		else return 0;
	}

}
