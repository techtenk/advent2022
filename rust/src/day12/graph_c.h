#include <stdint.h>

/**
 * Our aim here is to develop a somewhat generic interface for doing a All Pairs Shortest Path
 * The calling code should have already parsed and interpreted the data in the way that conforms to
 * the expectations of the API. The API will return a matrix that gives that shortest path from
 * every vertex to every vertex
*/

#ifndef advent_day12_bfs
#define advent_day12_bfs

/**
 * square_t will be our vertex
 * squares are index with x and y
*/
typedef struct square_s {
    uint32_t x;
    uint32_t y;
} square_t;

/**
 * pathedges keep a copy of the square, not a reference. To find the square in the graph
 * we will use the x,y because that's how squares are indexed. pathedges are directional,
 * so we will need to add one for each direction
 * 
*/
typedef struct pathedge_s {
    square_t start_square;
    square_t end_square;
} pathedge_t;

/**
 * graph provides the data structures needed (a graph consisting of squares and edges) and manages their 
 * memory allocation. member functions are in the form func_name(graph_t *self, ...)
*/
typedef struct graph_s graph_t;

enum FIND_RESULT_STATUS {
    FIND_RESULT_EMPTY = 1, // item was not found
    FIND_RESULT_ERROR = 2, // error other than just not found
    FIND_RESULT_RETURN_SQUARE = 4,
    FIND_RESULT_RETURN_PATHEDGE = 8
};

typedef struct find_result_s {
    enum FIND_RESULT_STATUS status;
    square_t square;
    pathedge_t pathedge;
} find_result_t;

/** create a new graph and return a pointer */
graph_t* create_graph();

/** destroy the graph and free associated memory, it will set self to a null pointer (void *) */
void destroy_graph(graph_t *self);

void add_square(graph_t *self, square_t new_square);
void add_pathedge(graph_t *self, pathedge_t new_edge);

find_result_t get_square(graph_t *self, uint32_t x, uint32_t y);

find_result_t get_pathedge(graph_t *self, square_t start, square_t end);

uint32_t count_squares(graph_t *self);

uint32_t djiksta_shortest_path(graph_t *self, square_t source);

uint32_t djiksta_single_dest_shortest_path(graph_t *self, square_t source, square_t dest);

uint32_t floyd_warshall_all_pairs_shortest_path(graph_t *self);

#endif