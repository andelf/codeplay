// Copyright (c) 2016 Copyright Holder All Rights Reserved.

#include <cstdint>
#include <cstddef>
#include <iostream>
#include <memory>
#include <fstream>
#include <algorithm>
#include <numeric>

#include <boost/config.hpp>
#include <boost/graph/adjacency_list.hpp>
#include <boost/graph/graphviz.hpp>
#include <boost/graph/prim_minimum_spanning_tree.hpp>
#include <boost/graph/kruskal_min_spanning_tree.hpp>


using namespace boost;

typedef adjacency_list<vecS, vecS, directedS, no_property,
  property<edge_weight_t, int64_t>> Graph;

typedef graph_traits<Graph>::vertex_descriptor vertex_descriptor;
typedef graph_traits<Graph>::edge_descriptor edge_descriptor;

int main(int argc, char const *argv[]) {
  std::ifstream fin("../priv/edges.txt");
  int num_of_nodes, num_of_edges;

  fin >> num_of_nodes >> num_of_edges;
  std::cout << "Nodes " << num_of_nodes << " Edges " << num_of_edges << std::endl;
  Graph g(num_of_nodes);
  // std::vector<int64_t> weights(num_of_nodes);

  for (int i=0; i < num_of_edges; ++i) {
    int u, v;
    int64_t weight;
    fin >> u >> v >> weight;
    // std::cout << u << " -> " << v <<std::endl;
    add_edge(u, v, weight, g);
  }

  // this use dijskura, don't allow negative edge weights
  // std::vector<vertex_descriptor> p(num_vertices(g));
  // prim_minimum_spanning_tree(g, &p[0]);

  std::vector<edge_descriptor> spanning_tree;
  kruskal_minimum_spanning_tree(g, std::back_inserter(spanning_tree));

  for (auto it = spanning_tree.begin(); it != spanning_tree.end(); ++it) {
    std::cout << source(*it, g) << " -- " << target(*it, g) << ";" << std::endl;
    // std::cout << get(edge_weight, g, *it) << std::endl;
  }

  int64_t mst_weights = std::accumulate(spanning_tree.begin(), spanning_tree.end(), 0,
      [g](int64_t acc, auto b) {
        return acc + get(edge_weight, g, b);
      });

  std::cout << "MST weights: " << mst_weights << std::endl;

  /*for (std::size_t i = 0; i != p.size(); ++i)
    if (p[i] != i)
      std::cout << "parent[" << i << "] = " << p[i] << std::endl;
    else
      std::cout << "parent[" << i << "] = no parent" << std::endl;
*/
  // write_graphviz(std::cout, g);
/*
  graph_traits<Graph>::edge_iterator ei, eend;
  for (boost::tie(ei, eend) = edges(g);  ei != eend; ++ei) {
    std::cout << "E: " << *ei << "   " << std::endl;
  }*/

  /*graph_traits<Graph>::vertex_iterator vi, vend;
  for (boost::tie(vi, vend) = vertices(g); vi != vend; ++vi) {
    std::cout << "V: " << *vi << std::endl;
  }*/

  //std::cout << num_edges(g) << std::endl;
  return 0;
}
