/**
Katy Williamson kcw4sn
Ben Terner bt3ze
@virginia.edu

Final Project for CS4414 - Operating Systems
A parallel MST implementation on images in Rust

*/


/*
some documentation:
Pixel object is composed of r,g,b values, a color, and an x,y index that takes the place of its adjacency list
Point is just an (x,y) value to denote a pixel's location
Edge contains two points: source and dest, along with the edge cost between them

*/


extern mod extra;
use std::{os, num, hashmap} ;
use extra::{arc,priority_queue};
use std::comm::*;
use extra::time;

struct Pixel{
    r: int,
    b: int,
    g: int,
    color: int,
    x: int,
    y: int
}

impl Pixel {
    fn new(red: int, blue: int, green: int, x: int, y: int) -> Pixel {
        Pixel{
            r: red,
            b: blue,
            g: green,
            color: -1,
            x: x,
            y: y
        }
    }
}

impl Clone for Pixel{
    fn clone(&self) -> Pixel {
        Pixel{
            r: self.r,
            b: self.b,
            g: self.g,
            color: self.color,
            x: self.x,
            y: self.y
        }
    }
}

struct Point {
    x: int,
    y: int
}

impl Point{
    fn new(x: int, y: int)-> Point {
        Point {
            x: x,
            y: y
        }
    }
}

struct Edge {
    source: Point,
    dest: Point,
    cost: float
}

impl Edge {
    fn new(s: Point, d: Point, cost: float) -> Edge {
        Edge {
            source: s,
            dest: d,
            cost: cost
        }
    }
}

impl Ord for Edge {
    fn lt(&self, other: &Edge) -> bool {
        self.cost > other.cost
    }
}

fn edgeCost(a: &Pixel, b: &Pixel) -> float {
    num::sqrt( (
            (a.r-b.r)*(a.r-b.r) + (a.g-b.g)*(a.g-b.g) + (a.b-b.b)*(a.b-b.b) ) as float)
}

fn findParent(map: &~[int], a: int) -> int {
    if map[a] < 0 {
            a
    } else {
        findParent(map, map[a])  
    }
}

fn setParent(colors: &mut ~[int], ind: int, val: int) {
    let tmp = colors[ind];
    colors[ind] = val;
    if tmp >= 0 {
        setParent(colors, tmp, val);
    }
}

fn readImage( filename: &~str) -> ~ [ ~[Pixel]] {
    let mut retval: ~[~[Pixel]] = ~[];

    let file: &str = filename.to_owned();
//    let file2: ~str = file.to_owned();
    let split_filename: ~[&str] = file.split_iter('.').collect();
    let file_data: &str = split_filename[0] + ".txt";

    println(fmt!("The image name is [%?]", file));
//    println(fmt!("The image data will be stored at [%?]", file_data));

    //read in the .txt associated with the file title
    let path = &PosixPath(file_data);

    match std::io::file_reader(path) {
	Ok(reader) => { 
	    //run the java program to process the image
/*	    let program: &str = "java ImageReader";
	    let argv: ~[~str] = ~[file2];
	    let mut prog = std::run::Process::new(program, argv, 
		std::run::ProcessOptions {
			env: None,
			dir: None,
			in_fd: None,
			out_fd: None,
			err_fd: None
		}
	    );
	    prog.finish();
*/
	    //parse out the array dimensions
    	    let dimensions_str: &str = reader.read_line();
   	    println(fmt!("dimensons: %s",dimensions_str));
	    let mut h: int = 0;
	    let mut w: int = 0;
	    match dimensions_str[0] as char {
		'H' => {
		    let split_dim: ~[&str] = dimensions_str.split_iter('W').collect();
		    h = from_str(split_dim[0].slice(1,split_dim[0].len())).unwrap();
		    w = from_str(split_dim[1]).unwrap();
		//    println(fmt!("Dimensions passed: H %? W %?",h,w));
		    //let retval: ~[ ~[Pixel, ..w], ..h];    
		}
		_ => {println("Dimensions corrupted! Abort");}
	    }

	    //fill in the retVal array with pixel data
	    for row in range(0,h){
                retval.push(~[]);
		for col in range(0,w){
	    	    if !reader.eof() {
			let line: &str = reader.read_line();
			//println(line);
			let rpos:uint = match line.find('R') { Some(n)=>n,None=>line.len() };
			let gpos:uint = match line.find('G') { Some(n)=>n,None=>line.len() };
			let bpos:uint = match line.find('B') { Some(n)=>n,None=>line.len() };
			
			if rpos == line.len() || gpos == line.len() || bpos == line.len() {
			    println("Pixel data corrupted! Abort");
			}

			let R:int = from_str(line.slice(rpos+1,gpos)).unwrap();
			let G:int = from_str(line.slice(gpos+1,bpos)).unwrap();
			let B:int = from_str(line.slice(bpos+1,line.len())).unwrap();

			//println(fmt!("R %? G %? B %?",R,G,B));

			//fn new(red: int, blue: int, green: int, x: int, y: int) -> Pixel {
			retval[row].push(Pixel::new(R,G,B,col,row));		

		    }
		    else {
			println("Data file corrupted! Abort");
		    }
		}
	    }
	},
	Err(err)   => { fail!(err) }
    }

    retval
}

fn main(){
    let argv =  os::args();

    let pixels: ~[ ~[Pixel] ] = readImage(&argv[1]);
    let height = pixels.len() as int;
    let width = pixels[0].len() as int;

    let starttime =  extra::time::precise_time_ns() as int;
    println(fmt!("Dimensions received: H %? W %?",height,width));

    let mut find_arcs: ~[ ~[arc::RWArc<Pixel>] ] =  ~[];
    for h in range(0,height){
	find_arcs.push(~[]);
        for w in range(0,width){
 //           println(fmt!("pix %?",pixels[h][w]));
	    find_arcs[h].push(arc::RWArc::new(pixels[h][w]));
	}
    }
    
    let arcs = find_arcs; // now we have an immutable array that can be shared
    
    let mut colormap: ~[int] = ~[];
//	let nodes =[(0,0),(width-1,height-1)];
//	let nodes =[(0,0),(width-1,height-1),(width/2,height/2)];
	let nodes =[(0,0),(width-1,0),(0,height-1),(width-1,height-1)];
//    let nodes = [ (0,0),(width-1,0),(0,height-1),(width-1,height-1), (width/2,height/2) ];
//	let nodes = [(0,0)];
   let numnodes: uint = nodes.len();
    println(fmt!("number of nodes: %u",numnodes));

    let mut color_index = 0;
    for &node in nodes.iter(){
        let (a,b) = node;
        do arcs[b][a].write |pix| {
            pix.color = color_index;
        }
        color_index+=1;
        colormap.push(-1);
    }
    
    let boundaryqueue: priority_queue::PriorityQueue<Edge> =  priority_queue::PriorityQueue::new();
    let boundary_arc: arc::RWArc<priority_queue::PriorityQueue<Edge>> = arc::RWArc::new(boundaryqueue);
    
    let (port, chan) = stream();
    let chan = SharedChan::new(chan);

    for &node in nodes.iter() {
        let (a,b) = node;
        let shared_arcs = arcs.clone();
        let shared_boundaries = boundary_arc.clone();
        let child_chan = chan.clone();
        do spawn { // split into threads here
            let mut x=a;
            let mut y=b;
            let mut queue: priority_queue::PriorityQueue<Edge> =  priority_queue::PriorityQueue::new();
            let mut visited: hashmap::HashMap<(int,int),bool> = hashmap::HashMap::new();            
            let mut newvisit: bool = true;
            let mut boundaries: priority_queue::PriorityQueue<Edge> =  priority_queue::PriorityQueue::new();
            let mut color: int = -1;
            do shared_arcs[b][a].read |pix| {
                color = pix.color;
            }
           
//            println(fmt!("start at (%i,%i): color = %i",c,d,color));
            loop {
                //  at most once each:
                //   visit the next vertex (determined by the last edge we chose or the starting vertex) to claim its neighbors and add its edges to our queue
                //   pop the next edge off the queue, which contains all of the edges between vertices claimed by the local thread
                
                
                if(newvisit){ // if the next vertex has not previously been visited to claim neighbors and add edges
                    let neighbors = [ (x,y-1),(x+1,y), (x,y+1), (x-1,y)]; // potential neighbors
                    for &coord in neighbors.iter() {
                        let (w,z) = coord;
                        if w >=0 && w < width && z >=0 && z < height { // bounds checking on neighbors to only select valid edges
                            do shared_arcs[z][w].read |dest| {
                                do shared_arcs[y][x].read |src| {
                                    let newedge = Edge::new(Point::new(x,y),Point::new(w,z),edgeCost(src,dest));
                                    if dest.color >= 0 {
                                        boundaries.push(newedge);         
                                    } else {
                                        queue.push(newedge);
                                    }
                                }
                            }
                        }
                    }
                }

                /*
                if(newvisit){ // if the next vertex has not previously been visited to claim neighbors and add edges
                    let neighbors = [ (x,y-1),(x+1,y), (x,y+1), (x-1,y)]; // potential neighbors
                    for &coord in neighbors.iter() {
                        let (w,z) = coord;
                        if w >=0 && w < width && z >=0 && z < height { // bounds checking on neighbors to only select valid edges
                            let mut newvertex = false;
                            let mut colored = false;
                            do shared_arcs[z][w].read |dest| {
                                if dest.color >= 0 {
                                    colored = true; //vertex has been previously claimed
                                }                         
                            }
                            if !colored { // this thread will claim the unclaimed vertex
                                do shared_arcs[z][w].write |dest| {
                                    if dest.color < 0 {
                                        dest.color = color;
                                        newvertex = true;
                                    } else {
                                        // edge has been claimed during a race since we last checked. add to supervertex boundary
                                        colored = true;
                                    }
                                }
                            }
                            if colored {
                                // we have found an edge that crosses a cut!
                                // here, somehow collect these edges for contraction later
                                do shared_arcs[y][x].read |src| {
                                    do shared_arcs[z][w].read |dest| {
                                        if dest.color != color {
                                            boundaries.push( Edge::new(Point::new(x,y),Point::new(w,z),edgeCost(src,dest)));
                                            // println(fmt!("%?\n-->%?",src,dest));
                                        }
                                    }
                                }  
                            }
                            if newvertex { // we have an unvisited but previously claimed vertex
                                // add the edge to our local edges queue
                                do shared_arcs[y][x].read |src| {
                                    do shared_arcs[z][w].read |dest| {
                                        queue.push( Edge::new(Point::new(x,y),Point::new(w,z),edgeCost(src,dest)));
                                    }
                                }
                            }
                        }
                    }
                }
                 */
                
                let edge = queue.maybe_pop(); // should fail only when we've exhausted all the edges in our queue, i.e. this thread is done running Prim
                match edge {
                    Some(e) => {
                        //println(fmt!("(%i,%i) pop (%i,%i)-%f-(%i,%i)",c,d,e.source.x,e.source.y,e.cost,e.dest.x,e.dest.y));
                        let coord = (e.dest.x,e.dest.y);
                        newvisit = false;
                        if !visited.contains_key(&coord){
                            // use visited hashmap to figure out if we're at a new vertex or a previously seen one
                            visited.insert(coord,true); //add to growing spanning forest
                            let (w,z) = coord;
                            let mut colored: bool = true;
                            do shared_arcs[z][w].write |dest| {
                                if dest.color < 0 {
                                    dest.color = color;
                                    colored = false;
                                }
                            }
                            if !colored {
                                x = w; // update search coordinates for next iteration
                                y = z;
                                newvisit = true; // update flag to claim new vertex's neighbors 
                                println(fmt!("(%i,%i),(%i,%i):%f\n",e.source.x,e.source.y,e.dest.x,e.dest.y,e.cost));
                            } else {
                                boundaries.push(e);
                            }
                        }
                    },
                    None => { break; } // this should execute at the end when there are no more unvisited or uncolored nodes for a thread
                }   
            }
            
            // write all of the supervertex's shared boundaries out to be compared concurrently for contraction
            do shared_boundaries.write |bounds| {
                loop {
                    match boundaries.maybe_pop() {
                        Some(e) => 
                            {
                            bounds.push(e);
                        }
                        None => { break; }
                    }
                }
            }
            child_chan.send(0); // tell master that we're done
        }
    }
    
    // wait for all the child threads to write their boundary-crossing edges
    for x in range(0,numnodes) {
        port.recv();
    }

    // then find the cut-crossing edges
    do boundary_arc.write |boundaries| {
        let mut bridges:uint = 0;
        loop {
            match boundaries.maybe_pop() {
                Some(e) => {
                     do arcs[e.dest.y][e.dest.x].read |dest| {
                        do arcs[e.source.y][e.source.x].read |src| {
//                            println(fmt!("boundary (%i,%i:%i)-%f-(%i,%i:%i)",src.x,src.y,src.color,e.cost,dest.x,dest.y,dest.color));       
                            let dest_parent = findParent(&colormap,dest.color);
                            let src_parent =  findParent(&colormap,src.color);
                            if src_parent != dest_parent {
                                setParent(&mut colormap,src.color,dest_parent);
                //                println(fmt!("bridge: (%i,%i:%i->%i)-(%i,%i:%i->%i) : %f",src.x,src.y,src.color,src_parent,dest.x,dest.y,dest.color,dest_parent, e.cost));
                                println(fmt!("(%i,%i),(%i,%i):%f",src.x,src.y,dest.x,dest.y,e.cost));
                                bridges +=1;
                            }
                        }    
                    }

                    // break when all of the supervertices have been connected
                    if bridges >= numnodes -1 {
                        break;
                    }
                },
                None => { break } // should not execute. break two lines above should do it
            }
        }
    }

    let endtime =  extra::time::precise_time_ns() as int;
    println(fmt!("elapsed time: %i ms",(endtime-starttime)/1000000));
  
}
