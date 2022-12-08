use petgraph::Graph;
use petgraph::adj::Neighbors;
use petgraph::graph::NodeIndex;
use petgraph::dot::{Dot,Config};
use std::collections::HashMap;
use petgraph::Undirected;
use rand::Rng;

fn main() {
    let mut graph = Graph::new_undirected();
    let nest = graph.add_node("nest");
    let food = graph.add_node("food");
    let mut nodes = Vec::new();
    let mut s = &1.to_string();
    let mut string = vec!["0","1","2","3","4","5","6","7","8","9"];
    for i in 0..10{
        let node = graph.add_node(string[i]);
        nodes.push(node);

    } 
    let mut edges :HashMap<(NodeIndex,NodeIndex), petgraph::prelude::EdgeIndex>= HashMap::new();

    //faire une matrice a la place

    edges.insert((nest, nodes[0]), graph.add_edge(nest, nodes[0], 1.0));
    edges.insert((nest, nodes[1]), graph.add_edge(nest, nodes[1], 1.0));
    edges.insert((nest, nodes[2]), graph.add_edge(nest, nodes[2], 1.0));
    edges.insert((nodes[2], nodes[3]), graph.add_edge(nodes[2], nodes[3], 1.0));
    edges.insert((nodes[3], food), graph.add_edge(nodes[3], food, 1.0));
    edges.insert((nodes[1], nodes[4]), graph.add_edge(nodes[1], nodes[4], 1.0));
    edges.insert((nodes[4], nodes[5]), graph.add_edge(nodes[4], nodes[5], 1.0));
    edges.insert((nodes[5], nodes[6]), graph.add_edge(nodes[5], nodes[6], 1.0));
    edges.insert((nodes[6], nodes[7]), graph.add_edge(nodes[6], nodes[7], 1.0));
    edges.insert((nodes[7], food), graph.add_edge(nodes[7], food, 1.0));
    edges.insert((nodes[0], nodes[8]), graph.add_edge(nodes[0], nodes[8], 1.0));
    edges.insert((nodes[8], nodes[9]), graph.add_edge(nodes[8], nodes[9], 1.0));
    edges.insert((nodes[9], food), graph.add_edge(nodes[9], food, 1.0));

    edges.insert(( nodes[0],nest), graph.add_edge(nest, nodes[0], 1.0));
    edges.insert((nodes[1],nest), graph.add_edge(nest, nodes[1], 1.0));
    edges.insert((nodes[2],nest), graph.add_edge(nest, nodes[2], 1.0));
    edges.insert((nodes[3], nodes[2]), graph.add_edge(nodes[2], nodes[3], 1.0));
    edges.insert((food,nodes[3]), graph.add_edge(nodes[3], food, 1.0));
    edges.insert((nodes[4], nodes[1]), graph.add_edge(nodes[1], nodes[4], 1.0));
    edges.insert((nodes[5], nodes[4]), graph.add_edge(nodes[4], nodes[5], 1.0));
    edges.insert((nodes[6], nodes[5]), graph.add_edge(nodes[5], nodes[6], 1.0));
    edges.insert((nodes[7], nodes[6]), graph.add_edge(nodes[6], nodes[7], 1.0));
    edges.insert((food, nodes[7]), graph.add_edge(nodes[7], food, 1.0));
    edges.insert((nodes[8], nodes[0]), graph.add_edge(nodes[0], nodes[8], 1.0));
    edges.insert((nodes[9], nodes[8]), graph.add_edge(nodes[8], nodes[9], 1.0));
    edges.insert(( food,nodes[9]), graph.add_edge( food,nodes[9], 1.0));

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeIndexLabel]));
    let mut pheromonematrix = vec![vec![0.0; 12]; 12];
    //dbg!(&edges);
    let depth = 25;
    ACO(&mut pheromonematrix, &graph, &edges, &nest, &food, depth);




    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeIndexLabel]));

}

#[derive(Clone)]
struct Ant {
    id: i32,
    location: NodeIndex,
    finished: bool,
    hasFood: bool,
    path: Vec<NodeIndex>,
    totalDistance: f64,
}

impl Ant{
    fn new(id: i32, location: NodeIndex) -> Self{
        Ant{
            id: id,
            location: location,
            finished: false,
            hasFood: false,
            path: Vec::new(),
            totalDistance: 0.0,
        }
    }
}

fn ACO(pheromoneMatrix: &mut Vec<Vec<f64>>, graph: &Graph<&str,f64,Undirected>, edges: &HashMap<(NodeIndex,NodeIndex), petgraph::prelude::EdgeIndex>, nest:&NodeIndex,food:&NodeIndex, depth : i32) {
    for i in 0..depth{
        let mut ants = vec![Ant::new(1, *nest),Ant::new(2, *nest), Ant::new(3, *nest), Ant::new(4, *nest), Ant::new(5, *nest), Ant::new(6, *nest)];
        //,Ant::new(2, *nest), Ant::new(3, *nest), Ant::new(4, *nest), Ant::new(5, *nest), Ant::new(6, *nest), Ant::new(7, *nest), Ant::new(8, *nest), Ant::new(9, *nest), Ant::new(10, *nest)
        //let mut pheromonematrix = pheromoneMatrix.clone();
    
        let mut runningAnts = ants.len();
    
        //while !ants.iter().fold( true, |acc,ant| acc&&ant.finished){  //marche pas a cause du borrow
        while runningAnts > 0{
            //dbg!("while");
            //for ant in ants.iter().filter(|x| x.finished).collect::<Vec<Ant>>(){  //probleme de reference chiant
            for i in 0..ants.len(){
                //let next_location = ;
                if !ants[i].finished{
                    ants[i].location= nextNode(ants[i].location, pheromoneMatrix, graph, edges, &mut ants[i]);
                    //dbg!(ants[i].location);
                    //dbg!(&ants[i].path);
                    if ants[i].location == *food && !ants[i].hasFood{
                        ants[i].hasFood = true;
                        ants[i].path = vec![*food];
                        dbg!("ant has food");
                    }
                    if ants[i].location == *nest && ants[i].hasFood{
                        ants[i].finished = true;
                        runningAnts-=1;
                        dbg!("ant finished");
                    }
                }
    
            }
        }
        depositPheromone(ants, pheromoneMatrix, edges);
        dbg!(&pheromoneMatrix);
    }

}

fn nextNode(currentNode: NodeIndex, pheromoneMatrix: &mut Vec<Vec<f64>>, graph: &Graph<&str,f64,Undirected>, edges:&HashMap<(NodeIndex,NodeIndex), petgraph::prelude::EdgeIndex>, ant:&mut Ant) -> NodeIndex{
    let alpha = 1.0;
    let beta = 1.0;
    let mut probabilities = Vec::new();
    let mut sum = 0.0;
    let mut Neighbors = Vec::new();
    //dbg!(&ant.path);
    //dbg!("aaaaaaaaaaaaaaa");
    //dbg!(currentNode);
    //dbg!(graph.neighbors(currentNode));
    for node in graph.neighbors(currentNode){
        if ant.path.contains(&node){
            continue;
        }
        //dbg!(currentNode, node);
        let value = pheromoneMatrix[currentNode.index()][node.index()].powf(alpha) + (graph.edge_weight(edges[&(currentNode, node)]).unwrap().to_owned() as f64).powf(-beta);
        sum += value;
        Neighbors.push(node);
        probabilities.push(value);
    }
    probabilities = probabilities.iter().map(|x| x/sum).collect::<Vec<f64>>();
    //dbg!(&probabilities);
    //select next node according to probabilities
    let mut rng = rand::thread_rng();
    let x = probabilities.iter().fold(0.0, |acc, x| acc + x*100.0);
    dbg!(x);
    let mut random = rng.gen_range(0..x as i64) as f64;
    dbg!(random);
    dbg!(&probabilities);
    let mut i = 0;
    while random > probabilities[i]*100.0{
        random -= probabilities[i]*100.0;
        i += 1;
    }
    //let i = rng.gen_range(0..probabilities.len());
    ant.path.push(Neighbors[i]);
    ant.totalDistance += graph.edge_weight(edges[&(currentNode, Neighbors[i])]).unwrap().to_owned() as f64;
    //ant.location = graph.neighbors(currentNode).nth(i).unwrap();
    return Neighbors[i];
}

fn depositPheromone( ants: Vec<Ant>, pheromoneMatrix: &mut Vec<Vec<f64>>, edges: &HashMap<(NodeIndex,NodeIndex),petgraph::prelude::EdgeIndex>){
    let rho = 0.5;
    //dbg!("aaaaaaaaaaaaaa");
    for ant in ants{
        for i in 0..ant.path.len()-2{
            let value = (1.0-rho)*pheromoneMatrix[ant.path[i].index()][ant.path[i+1].index()] + 1.0/ant.totalDistance;
            dbg!((1.0-rho)*pheromoneMatrix[ant.path[i].index()][ant.path[i+1].index()]);
            pheromoneMatrix[ant.path[i].index()][ant.path[i+1].index()] = value;
            pheromoneMatrix[ant.path[i+1].index()][ant.path[i].index()] = value;
        }
    }
}