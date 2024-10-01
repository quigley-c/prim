// Carson Quigley - CS 575 Prim's Algorithm

use std::io;

#[derive(Clone)]
struct Heap {
    indicies: Vec<usize>,
    data: Vec<Vert>,
}

#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
    val: usize,
}

#[derive(Clone)]
struct Vert {
    name: usize,
    edges: Vec<usize>,
    label: i32,
}

fn main() {
    // read stdin
    let lines: Vec<String> = io::stdin()
        .lines()
        .collect::<Result<_,_>>()
        .unwrap();
    let tuple = init_heap(lines);

    let start_v = 0; // No need for user input here this time.
    let result = prim(tuple.0.clone(), tuple.1.clone(), start_v);
    if result == -1 { println!("not connected"); } else {
        println!("{}", result);
    }
}

fn prim(mut heap: Heap, edges: Vec<Edge>, start_v: usize) -> i32 {
    /* For Prim, as opposed to Dijkstra, we keep track of the total distance
     * traveled ove the whole graph instead of the shortest path between 2 nodes.
     * To this end, we modify the loop to compare neighboring edge weights rather
     * than cumulative weights. */
    decrease_key(&mut heap, start_v, 0);
    let mut dist: i32 = 0; // this val

    while heap.data.len() > 0 {
        // we set the start vertex as having distance 0 in advance (hopefully)
        let v = heap_extract(&mut heap).expect("");
        sift_down(&mut heap, 0);

        // we popped the next shortest distance so we know it's in the minimum tree.
        dist += v.label;

        if v.label == i32::MAX { return dist; } // useful for unconnected pairs

        for e in v.edges {
            if heap.indicies[edges[e].to] >= heap.data.len() {
                continue;
            } // we've popped the node
            let new_dist = edges[e].val as i32;
            if new_dist < heap.data[heap.indicies[edges[e].to]].label {
                let vi = heap.data[heap.indicies[edges[e].to]].clone();
                decrease_key(&mut heap, vi.name, new_dist);
            }
        }
    }

    return dist;
}

fn sift_down(heap: &mut Heap, i: usize) {
    // check the left and right children for smaller nodes, reverse of perc_up
    // useful after popping a node off the Vec structure.
    // Since only the front is guaranteed to be smallest the tree must be
    // rebalanced from the top down
    fn left(i:usize) -> usize {2*i+1}
    fn right(i:usize) -> usize {2*i+2}

    let mut smallest = i;
    if left(i) < heap.data.len() &&
        heap.data[left(i)].label < heap.data[smallest].label {
        smallest = left(i);
    }
    if right(i) < heap.data.len() &&
        heap.data[right(i)].label < heap.data[smallest].label {
        smallest = right(i);
    }
    if smallest != i {
        heap_swap(heap, i, smallest);
        sift_down(heap, smallest);
    }
}

fn heap_extract(heap: &mut Heap) -> Result<Vert, &str>{
    // preps the heap for extraction
    // by swapping the smallest node to the front of the queue
    // this simplifies popping from the queue so we don't have
    // to recompute the indexes, letting the pop() function handle
    // the Vec management for us
    if heap.data.len() == 0 { return Err("nope"); };

    let start_i = 0;
    let len = heap.data.len();

    heap_swap(heap, start_i, len - 1);
    let v = heap.data.pop().unwrap();
    return Ok(v);
}

fn decrease_key(heap: &mut Heap, key: usize, val: i32) {
    // updates the node[key] in the tree with the new value.
    // ensures that the tree is rebalanced afterwrads with perc_up()
    let index = heap.indicies[key].clone();
    heap.data[index].label = val;
    perc_up(heap, index);
}

fn heap_swap(heap: &mut Heap, vi: usize, pi: usize) {
    // simple swap func that updates the indexes Vec
    let tmp = heap.data[vi].clone();
    heap.data[vi] = heap.data[pi].clone();
    heap.data[pi] = tmp;

    let tmp = heap.indicies[heap.data[vi].name].clone();
    heap.indicies[heap.data[vi].name] = heap.indicies[heap.data[pi].name];
    heap.indicies[heap.data[pi].name] = tmp;
}

fn perc_up(heap: &mut Heap, vi: usize) {
    // Tree balancing from the bottom up. reverse of sift_down
    if vi == 0 { return; }
    let parent_i = (vi-1)/2;
    if heap.data[vi].label < heap.data[parent_i].label {
        heap_swap(heap, vi, parent_i);
        perc_up(heap, parent_i);
    }
}

fn heap_insert(heap: &mut Heap, v: Vert) {
    // helper that manages the heap fields
    let index = heap.data.len();
    heap.indicies.push(index);
    heap.data.push(v.clone());

    perc_up(heap, index);
}

fn init_heap(lines: Vec<String>) -> (Heap, Vec<Edge>) {
    // builds heap struct from a Vec of (space separated ints) as Strings
    let parts: Vec<usize> = lines[0]
        .split(" ")
        .filter_map(|w| w.parse().ok())
        .collect();

    let len_v: usize = parts[0];
    let len_e: usize = parts[1];
    let mut heap: Heap = Heap {
        indicies: vec![],
        data: vec![],
    };

    for i in 0..len_v {
        let v: Vert = Vert {
            name: i,
            edges: <Vec<usize>>::new(),
            label: i32::MAX
        };
        heap_insert(&mut heap, v);
    }

    let mut edges: Vec<Edge> = Vec::with_capacity(len_e);
    let mut edge_index: usize = 0;
    // rust syntax still confounds me
    for l in lines {
        let nums: Vec<_> = l.split(" ")
            .filter_map(|w| w.parse().ok())
            .collect();
        if nums.len() != 3 {
            // we don't need the first line
            continue;
        }
        let e: Edge = Edge {
            from: nums[0],
            to: nums[1],
            val: nums[2],
        };

        heap.data[e.from].edges.push(edge_index);
        edges.push(e);
        edge_index += 1;
    }

    return (heap, edges);
}

