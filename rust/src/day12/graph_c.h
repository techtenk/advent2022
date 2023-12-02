#include <stdint.h>

#include "graph_c_base.h"
#include "graph_c_options.h"

/**
 * Our aim here is to develop a somewhat generic interface for doing a All Pairs Shortest Path
 * The calling code should have already parsed and interpreted the data in the way that conforms to
 * the expectations of the API. The API will return a matrix that gives that shortest path from
 * every vertex to every vertex
*/

#ifndef GRAPH_C
#define GRAPH_C

/** create a new graph and return a pointer */
graph_t* create_graph();

/** destroy the graph and free associated memory, it will set self to a null pointer (void *) */
void destroy_graph(graph_t *self);

insert_result_t add_square(graph_t *self, square_t new_square);
insert_result_t add_pathedge(graph_t *self, pathedge_t new_edge);

find_result_t get_square(graph_t *self, uint32_t x, uint32_t y);

find_result_t get_pathedge(graph_t *self, square_t start, square_t end);

uint32_t count_squares(graph_t *self);

uint32_t djiksta_shortest_path(graph_t *self, square_t source);

uint32_t djiksta_single_dest_shortest_path(graph_t *self, square_t source, square_t dest);

uint32_t floyd_warshall_all_pairs_shortest_path(graph_t *self);

#endif