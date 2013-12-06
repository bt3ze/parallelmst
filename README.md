cs4414-mst
==========
Katy Williamson and Ben Terner
kcw4sn, bt3ze @virginia.edu
==========

Project Description:
Our project is a parallel implementation of a Minimum Spanning Tree algorithm in the Rust programming language. Conceptually, the implemented algorithm is a combination of the much-studied Prim and Boruvka algorithms, with concepts borrowed liberally from http://www.cc.gatech.edu/~bader/papers/MST-JPDC.pdf. The program makes use of X parallel threads in order to construct an accurate MST from a graph (note: our "MST" in the current implementation is a very close approximation. The fix for a true MST is known but has not been implemented.). In each thread, from a starting node, the program will repeatedly visit each of the node’s neighbors, choose the lightest weight edge, and contract that node into a tree. The program completes when all nodes have been visited and the X threads (X trees in a forest) contract into a single tree. 

We applied this procedure to graphs consisting of pixels in a rectangular image. By producing an MST of image data, it’s possible to easily sort and manipulate the pixels of the edges in the spanning tree for interesting image processing applications. If many heavy-weight edges are highlighted, the edges between areas of different colors can be detected (See gradient output below). If the bounded sections in an image are easily detectable, automating object detection or facial detection can be achieved.


===========
Instructions for usage

create image data file:
java ImageReader filename.jpg

compile mst:
rustc mst.rs

run mst:
./mst filename.jpg  (must be done after creating the .txt data file using ImageReader)

run visualizer:
java MSTVisualizer outfile.txt (where outfile was produced by capturing stdout from mst)

check the outputImages folder
